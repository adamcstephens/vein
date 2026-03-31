use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tokio::sync::mpsc;

use crate::client::{Task, VikunjaClient};
use crate::project::{BoardState, ProjectClient};

const POLL_INTERVAL: Duration = Duration::from_secs(5);

struct App {
    board: BoardState,
    selected_column: usize,
    list_states: [ListState; 3],
    last_refresh: Instant,
    poll_error: Option<String>,
}

impl App {
    fn new() -> Self {
        App {
            board: BoardState {
                ready: vec![],
                in_progress: vec![],
                done: vec![],
            },
            selected_column: 0,
            list_states: [
                ListState::default(),
                ListState::default(),
                ListState::default(),
            ],
            last_refresh: Instant::now(),
            poll_error: None,
        }
    }

    fn column_tasks(&self, col: usize) -> &[Task] {
        match col {
            0 => &self.board.ready,
            1 => &self.board.in_progress,
            2 => &self.board.done,
            _ => &[],
        }
    }

    fn update_board(&mut self, board: BoardState) {
        self.board = board;
        self.last_refresh = Instant::now();
        self.poll_error = None;
        // Clamp selections to new list lengths
        for col in 0..3 {
            let len = self.column_tasks(col).len();
            if len == 0 {
                self.list_states[col].select(None);
            } else if let Some(i) = self.list_states[col].selected()
                && i >= len
            {
                self.list_states[col].select(Some(len - 1));
            }
        }
    }

    fn move_down(&mut self) {
        let col = self.selected_column;
        let len = self.column_tasks(col).len();
        if len == 0 {
            return;
        }
        let i = match self.list_states[col].selected() {
            Some(i) => (i + 1).min(len - 1),
            None => 0,
        };
        self.list_states[col].select(Some(i));
    }

    fn move_up(&mut self) {
        let col = self.selected_column;
        let len = self.column_tasks(col).len();
        if len == 0 {
            return;
        }
        let i = match self.list_states[col].selected() {
            Some(i) => i.saturating_sub(1),
            None => 0,
        };
        self.list_states[col].select(Some(i));
    }

    fn move_left(&mut self) {
        if self.selected_column > 0 {
            self.selected_column -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.selected_column < 2 {
            self.selected_column += 1;
        }
    }
}

fn format_task_item(task: &Task) -> ListItem<'static> {
    let priority_indicator = match task.priority {
        4 => "!! ",
        3 => "!  ",
        2 => "*  ",
        1 => ".  ",
        _ => "   ",
    };

    let labels = if task.labels.is_empty() {
        String::new()
    } else {
        format!(
            " [{}]",
            task.labels
                .iter()
                .map(|l| l.title.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    let text = format!(
        "{}{}: {}{}",
        priority_indicator,
        task.display_id(),
        task.title,
        labels
    );
    ListItem::new(text)
}

fn draw(frame: &mut ratatui::Frame, app: &mut App) {
    let outer = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(frame.area());
    let columns = Layout::horizontal([
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
    ])
    .split(outer[0]);

    let titles = [
        format!("Ready ({})", app.board.ready.len()),
        format!("In Progress ({})", app.board.in_progress.len()),
        format!("Done ({})", app.board.done.len()),
    ];

    for (col, area) in columns.iter().enumerate() {
        let is_active = col == app.selected_column;
        let border_style = if is_active {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(titles[col].as_str())
            .borders(Borders::ALL)
            .border_style(border_style);

        let items: Vec<ListItem> = app.column_tasks(col).iter().map(format_task_item).collect();

        let highlight_style = if is_active {
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let list = List::new(items)
            .block(block)
            .highlight_style(highlight_style)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, *area, &mut app.list_states[col]);
    }

    // Status bar
    let elapsed = app.last_refresh.elapsed().as_secs();
    let status_text = if let Some(err) = &app.poll_error {
        vec![
            Span::styled(format!(" Error: {err} "), Style::default().fg(Color::Red)),
            Span::raw(format!(" | {elapsed}s ago")),
        ]
    } else {
        vec![
            Span::raw(format!(" Updated {elapsed}s ago")),
            Span::styled(
                " | q: quit  j/k: up/down  h/l: columns",
                Style::default().fg(Color::DarkGray),
            ),
        ]
    };

    let status_bar = Paragraph::new(Line::from(status_text));
    frame.render_widget(status_bar, outer[1]);
}

enum PollResult {
    Ok(BoardState),
    Err(String),
}

pub async fn run<C: VikunjaClient + Clone + Send + Sync + 'static>(
    project: ProjectClient<C>,
) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, project).await;

    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    result
}

async fn run_loop<C: VikunjaClient + Clone + Send + Sync + 'static>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    project: ProjectClient<C>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    // Initial fetch
    match project.list_board().await {
        Ok(board) => app.update_board(board),
        Err(e) => app.poll_error = Some(e.to_string()),
    }

    let (tx, mut rx) = mpsc::channel::<PollResult>(1);

    // Spawn background poller
    let poll_project = project.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(POLL_INTERVAL).await;
            let result = match poll_project.list_board().await {
                Ok(board) => PollResult::Ok(board),
                Err(e) => PollResult::Err(e.to_string()),
            };
            if tx.send(result).await.is_err() {
                break;
            }
        }
    });

    loop {
        terminal.draw(|frame| draw(frame, &mut app))?;

        // Poll for events with a short timeout so we can check the channel
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('j') | KeyCode::Down => app.move_down(),
                KeyCode::Char('k') | KeyCode::Up => app.move_up(),
                KeyCode::Char('h') | KeyCode::Left => app.move_left(),
                KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => app.move_right(),
                KeyCode::BackTab => app.move_left(),
                _ => {}
            }
        }

        // Check for poll updates
        while let Ok(result) = rx.try_recv() {
            match result {
                PollResult::Ok(board) => app.update_board(board),
                PollResult::Err(e) => app.poll_error = Some(e),
            }
        }
    }

    Ok(())
}
