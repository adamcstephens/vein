use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use tokio::sync::mpsc;

use crate::client::{Label, Task, VikunjaClient};
use crate::project::{BoardState, ProjectClient};

const POLL_INTERVAL: Duration = Duration::from_secs(5);

const PRIORITY_NAMES: &[&str] = &["None", "Low", "Medium", "High", "Urgent"];

enum Mode {
    Board,
    Detail,
    CreateTask,
    EditTask,
    ConfirmDiscard,
}

#[derive(Clone, Copy, PartialEq)]
enum FormField {
    Title,
    Description,
    Priority,
    Labels,
}

const FORM_FIELDS: &[FormField] = &[
    FormField::Title,
    FormField::Description,
    FormField::Priority,
    FormField::Labels,
];

struct TaskForm {
    title: String,
    description: String,
    priority: usize, // index into PRIORITY_NAMES
    available_labels: Vec<Label>,
    selected_labels: Vec<bool>,
    active_field: FormField,
    label_cursor: usize,
    editing_task_id: Option<String>,
    original_label_ids: Vec<i64>,
}

impl TaskForm {
    fn new(labels: Vec<Label>) -> Self {
        let label_count = labels.len();
        TaskForm {
            title: String::new(),
            description: String::new(),
            priority: 0,
            available_labels: labels,
            selected_labels: vec![false; label_count],
            active_field: FormField::Title,
            label_cursor: 0,
            editing_task_id: None,
            original_label_ids: vec![],
        }
    }

    fn from_task(task: &Task, labels: Vec<Label>) -> Self {
        let task_label_ids: Vec<i64> = task.labels.iter().map(|l| l.id).collect();
        let selected: Vec<bool> = labels
            .iter()
            .map(|l| task_label_ids.contains(&l.id))
            .collect();
        let description = crate::markdown::html_to_markdown(&task.description)
            .unwrap_or_else(|_| task.description.clone());
        TaskForm {
            title: task.title.clone(),
            description: description.trim_end().to_string(),
            priority: task.priority as usize,
            available_labels: labels,
            selected_labels: selected,
            active_field: FormField::Title,
            label_cursor: 0,
            editing_task_id: Some(task.display_id()),
            original_label_ids: task_label_ids,
        }
    }

    fn is_empty(&self) -> bool {
        self.title.is_empty()
            && self.description.is_empty()
            && self.priority == 0
            && self.selected_labels.iter().all(|&s| !s)
    }

    fn next_field(&mut self) {
        let idx = FORM_FIELDS
            .iter()
            .position(|f| *f == self.active_field)
            .unwrap_or(0);
        let next = (idx + 1) % FORM_FIELDS.len();
        self.active_field = FORM_FIELDS[next];
    }

    fn prev_field(&mut self) {
        let idx = FORM_FIELDS
            .iter()
            .position(|f| *f == self.active_field)
            .unwrap_or(0);
        let prev = if idx == 0 {
            FORM_FIELDS.len() - 1
        } else {
            idx - 1
        };
        self.active_field = FORM_FIELDS[prev];
    }

    fn priority_value(&self) -> Option<i64> {
        if self.priority == 0 {
            None
        } else {
            Some(self.priority as i64)
        }
    }

    fn selected_label_ids(&self) -> Vec<i64> {
        self.available_labels
            .iter()
            .zip(self.selected_labels.iter())
            .filter(|&(_, &selected)| selected)
            .map(|(label, _)| label.id)
            .collect()
    }
}

struct App {
    board: BoardState,
    selected_column: usize,
    list_states: [ListState; 3],
    last_refresh: Instant,
    poll_error: Option<String>,
    detail_task: Option<Task>,
    detail_scroll: u16,
    mode: Mode,
    task_form: Option<TaskForm>,
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
            detail_task: None,
            detail_scroll: 0,
            mode: Mode::Board,
            task_form: None,
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

    fn selected_task(&self) -> Option<&Task> {
        let col = self.selected_column;
        let tasks = self.column_tasks(col);
        self.list_states[col].selected().and_then(|i| tasks.get(i))
    }

    fn open_detail(&mut self) {
        if let Some(task) = self.selected_task() {
            self.detail_task = Some(task.clone());
            self.detail_scroll = 0;
            self.mode = Mode::Detail;
        }
    }

    fn close_detail(&mut self) {
        self.detail_task = None;
        self.detail_scroll = 0;
        self.mode = Mode::Board;
    }

    fn start_create(&mut self, labels: Vec<Label>) {
        self.task_form = Some(TaskForm::new(labels));
        self.mode = Mode::CreateTask;
    }

    fn start_edit(&mut self, task: &Task, labels: Vec<Label>) {
        self.task_form = Some(TaskForm::from_task(task, labels));
        self.mode = Mode::EditTask;
    }

    fn close_form(&mut self) {
        self.task_form = None;
        self.mode = Mode::Board;
    }

    fn try_close_form(&mut self) {
        if let Some(form) = &self.task_form {
            if form.is_empty() {
                self.close_form();
            } else {
                self.mode = Mode::ConfirmDiscard;
            }
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
                " | q: quit  j/k: up/down  h/l: columns  o: open  c: create  e: edit",
                Style::default().fg(Color::DarkGray),
            ),
        ]
    };

    let status_bar = Paragraph::new(Line::from(status_text));
    frame.render_widget(status_bar, outer[1]);

    // Task detail overlay
    if let Some(task) = &app.detail_task {
        let area = frame.area();
        let popup_width = (area.width * 3 / 4).max(40).min(area.width);
        let popup_height = (area.height * 3 / 4).max(10).min(area.height);
        let x = (area.width.saturating_sub(popup_width)) / 2;
        let y = (area.height.saturating_sub(popup_height)) / 2;
        let popup_area = ratatui::layout::Rect::new(x, y, popup_width, popup_height);

        let lines = build_detail_lines(task);
        let block = Block::default()
            .title(format!(
                "{}: {} (Esc to close, j/k to scroll, e to edit)",
                task.display_id(),
                task.title
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((app.detail_scroll, 0));

        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);
    }

    // Task form overlay (create or edit)
    if matches!(
        app.mode,
        Mode::CreateTask | Mode::EditTask | Mode::ConfirmDiscard
    ) && let Some(form) = &app.task_form
    {
        let form_title = if form.editing_task_id.is_some() {
            "Edit Task"
        } else {
            "New Task"
        };
        draw_task_form(
            frame,
            form,
            form_title,
            matches!(app.mode, Mode::ConfirmDiscard),
        );
    }
}

fn draw_task_form(frame: &mut ratatui::Frame, form: &TaskForm, title: &str, confirming: bool) {
    let area = frame.area();
    let popup_width = (area.width * 4 / 5).max(50).min(area.width);
    let popup_height = (area.height * 4 / 5).max(16).min(area.height);
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = ratatui::layout::Rect::new(x, y, popup_width, popup_height);

    frame.render_widget(Clear, popup_area);

    let outer_block = Block::default()
        .title(format!(
            "{title} (Ctrl+S: save, Esc: cancel, Tab: next field)"
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));
    let inner = outer_block.inner(popup_area);
    frame.render_widget(outer_block, popup_area);

    // Calculate label rows needed
    let label_rows = form.available_labels.len().max(1) as u16;
    let desc_min = 3u16;

    let field_layout = Layout::vertical([
        Constraint::Length(3),                     // Title
        Constraint::Min(desc_min),                 // Description
        Constraint::Length(3),                     // Priority
        Constraint::Length(label_rows.min(6) + 2), // Labels
    ])
    .split(inner);

    // Title field
    let title_style = field_border_style(form.active_field == FormField::Title);
    let title_block = Block::default()
        .title("Title")
        .borders(Borders::ALL)
        .border_style(title_style);
    let title_text = if form.active_field == FormField::Title {
        format!("{}\u{2588}", form.title)
    } else {
        form.title.clone()
    };
    let title_widget = Paragraph::new(title_text).block(title_block);
    frame.render_widget(title_widget, field_layout[0]);

    // Description field
    let desc_style = field_border_style(form.active_field == FormField::Description);
    let desc_block = Block::default()
        .title("Description")
        .borders(Borders::ALL)
        .border_style(desc_style);
    let desc_text = if form.active_field == FormField::Description {
        format!("{}\u{2588}", form.description)
    } else {
        form.description.clone()
    };
    let desc_widget = Paragraph::new(desc_text)
        .block(desc_block)
        .wrap(Wrap { trim: false });
    frame.render_widget(desc_widget, field_layout[1]);

    // Priority field
    let prio_style = field_border_style(form.active_field == FormField::Priority);
    let prio_block = Block::default()
        .title("Priority (Left/Right to change)")
        .borders(Borders::ALL)
        .border_style(prio_style);
    let prio_text = format!("< {} >", PRIORITY_NAMES[form.priority]);
    let prio_widget = Paragraph::new(prio_text).block(prio_block);
    frame.render_widget(prio_widget, field_layout[2]);

    // Labels field
    let label_style = field_border_style(form.active_field == FormField::Labels);
    let label_block = Block::default()
        .title("Labels (Space to toggle)")
        .borders(Borders::ALL)
        .border_style(label_style);

    if form.available_labels.is_empty() {
        let label_widget = Paragraph::new("  No labels available").block(label_block);
        frame.render_widget(label_widget, field_layout[3]);
    } else {
        let label_items: Vec<ListItem> = form
            .available_labels
            .iter()
            .enumerate()
            .map(|(i, label)| {
                let check = if form.selected_labels[i] {
                    "[x]"
                } else {
                    "[ ]"
                };
                let prefix = if form.active_field == FormField::Labels && i == form.label_cursor {
                    "> "
                } else {
                    "  "
                };
                ListItem::new(format!("{prefix}{check} {}", label.title))
            })
            .collect();
        let label_list = List::new(label_items).block(label_block);
        frame.render_widget(label_list, field_layout[3]);
    }

    // Confirm discard dialog
    if confirming {
        let dialog_width = 40u16.min(area.width);
        let dialog_height = 5u16;
        let dx = (area.width.saturating_sub(dialog_width)) / 2;
        let dy = (area.height.saturating_sub(dialog_height)) / 2;
        let dialog_area = ratatui::layout::Rect::new(dx, dy, dialog_width, dialog_height);

        frame.render_widget(Clear, dialog_area);
        let dialog_block = Block::default()
            .title("Discard changes?")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red));
        let dialog = Paragraph::new("  y: discard  n: keep editing").block(dialog_block);
        frame.render_widget(dialog, dialog_area);
    }
}

fn build_detail_lines(task: &Task) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    // Status
    let status_style = if task.done {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Yellow)
    };
    let status_text = if task.done { "Done" } else { "Open" };
    lines.push(Line::from(vec![
        Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
        Span::styled(status_text.to_string(), status_style),
    ]));

    // Priority
    let (priority_name, priority_style) = match task.priority {
        4 => (
            "Urgent",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        3 => ("High", Style::default().fg(Color::Red)),
        2 => ("Medium", Style::default().fg(Color::Yellow)),
        1 => ("Low", Style::default().fg(Color::Blue)),
        _ => ("None", Style::default().fg(Color::DarkGray)),
    };
    if task.priority > 0 {
        lines.push(Line::from(vec![
            Span::styled("Priority: ", Style::default().fg(Color::DarkGray)),
            Span::styled(priority_name.to_string(), priority_style),
        ]));
    }

    // Labels
    if !task.labels.is_empty() {
        let mut spans = vec![Span::styled(
            "Labels: ",
            Style::default().fg(Color::DarkGray),
        )];
        for (i, label) in task.labels.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw(", "));
            }
            spans.push(Span::styled(
                label.title.clone(),
                Style::default().fg(Color::Magenta),
            ));
        }
        lines.push(Line::from(spans));
    }

    // Assignees
    if !task.assignees.is_empty() {
        let mut spans = vec![Span::styled(
            "Assignees: ",
            Style::default().fg(Color::DarkGray),
        )];
        for (i, user) in task.assignees.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw(", "));
            }
            let name = if user.name.is_empty() {
                user.username.clone()
            } else {
                user.name.clone()
            };
            spans.push(Span::styled(name, Style::default().fg(Color::Cyan)));
        }
        lines.push(Line::from(spans));
    }

    // Relations
    if !task.related_tasks.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Relations",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        for (kind, related) in &task.related_tasks {
            for t in related {
                let status_indicator = if t.done { " ✓" } else { "" };
                let rel_style = if t.done {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default()
                };
                lines.push(Line::from(vec![
                    Span::styled(format!("  {kind}: "), Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{} {}", t.display_id(), t.title), rel_style),
                    Span::styled(
                        status_indicator.to_string(),
                        Style::default().fg(Color::Green),
                    ),
                ]));
            }
        }
    }

    // Description
    if !task.description.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Description",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        let description = crate::markdown::html_to_markdown(&task.description)
            .unwrap_or_else(|_| task.description.clone());
        for line in description.trim_end().lines() {
            lines.push(Line::from(line.to_string()));
        }
    }

    lines
}

fn field_border_style(active: bool) -> Style {
    if active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

enum AppEvent {
    BoardUpdate(BoardState),
    PollError(String),
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

    let (tx, mut rx) = mpsc::channel::<AppEvent>(4);

    // Spawn background poller
    let poll_tx = tx.clone();
    let poll_project = project.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(POLL_INTERVAL).await;
            let event = match poll_project.list_board().await {
                Ok(board) => AppEvent::BoardUpdate(board),
                Err(e) => AppEvent::PollError(e.to_string()),
            };
            if poll_tx.send(event).await.is_err() {
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
            match app.mode {
                Mode::Detail => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => app.close_detail(),
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.detail_scroll = app.detail_scroll.saturating_add(1);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.detail_scroll = app.detail_scroll.saturating_sub(1);
                    }
                    KeyCode::Char('e') => {
                        if let Some(task) = app.detail_task.clone() {
                            app.close_detail();
                            let labels = project.list_labels().await.unwrap_or_default();
                            app.start_edit(&task, labels);
                        }
                    }
                    _ => {}
                },
                Mode::ConfirmDiscard => match key.code {
                    KeyCode::Char('y') => app.close_form(),
                    KeyCode::Char('n') | KeyCode::Esc => {
                        // Return to whichever form mode we came from
                        app.mode = if app
                            .task_form
                            .as_ref()
                            .is_some_and(|f| f.editing_task_id.is_some())
                        {
                            Mode::EditTask
                        } else {
                            Mode::CreateTask
                        };
                    }
                    _ => {}
                },
                Mode::CreateTask | Mode::EditTask => {
                    // Ctrl+S to save
                    if key.code == KeyCode::Char('s')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        if let Some(form) = app.task_form.take() {
                            app.mode = Mode::Board;
                            let title = form.title.trim().to_string();
                            if !title.is_empty() {
                                let desc = if form.description.trim().is_empty() {
                                    None
                                } else {
                                    Some(form.description.as_str())
                                };
                                if let Some(task_ref) = &form.editing_task_id {
                                    // Edit existing task
                                    match project
                                        .update_task(
                                            task_ref,
                                            Some(title),
                                            desc.map(String::from),
                                            Some(form.priority as i64),
                                        )
                                        .await
                                    {
                                        Ok(_) => {
                                            // Reconcile labels: add new, remove old
                                            let new_ids = form.selected_label_ids();
                                            for label_id in &new_ids {
                                                if !form.original_label_ids.contains(label_id) {
                                                    let _ = project
                                                        .add_label(task_ref, *label_id)
                                                        .await;
                                                }
                                            }
                                            // Note: Vikunja API doesn't support removing labels
                                            // via the current client, so only additions are applied
                                            if let Ok(board) = project.list_board().await {
                                                app.update_board(board);
                                            }
                                        }
                                        Err(e) => app.poll_error = Some(e.to_string()),
                                    }
                                } else {
                                    // Create new task
                                    match project
                                        .create_task(&title, desc, form.priority_value())
                                        .await
                                    {
                                        Ok(task) => {
                                            for label_id in form.selected_label_ids() {
                                                let id_str = task.display_id();
                                                let _ = project.add_label(&id_str, label_id).await;
                                            }
                                            if let Ok(board) = project.list_board().await {
                                                app.update_board(board);
                                            }
                                        }
                                        Err(e) => app.poll_error = Some(e.to_string()),
                                    }
                                }
                            }
                        }
                    } else if let Some(form) = &mut app.task_form {
                        match key.code {
                            KeyCode::Esc => app.try_close_form(),
                            KeyCode::Tab => form.next_field(),
                            KeyCode::BackTab => form.prev_field(),
                            _ => match form.active_field {
                                FormField::Title => match key.code {
                                    KeyCode::Backspace => {
                                        form.title.pop();
                                    }
                                    KeyCode::Char(c) => form.title.push(c),
                                    _ => {}
                                },
                                FormField::Description => match key.code {
                                    KeyCode::Backspace => {
                                        form.description.pop();
                                    }
                                    KeyCode::Enter => form.description.push('\n'),
                                    KeyCode::Char(c) => form.description.push(c),
                                    _ => {}
                                },
                                FormField::Priority => match key.code {
                                    KeyCode::Left | KeyCode::Char('h') => {
                                        form.priority = form.priority.saturating_sub(1);
                                    }
                                    KeyCode::Right | KeyCode::Char('l') => {
                                        form.priority =
                                            (form.priority + 1).min(PRIORITY_NAMES.len() - 1);
                                    }
                                    _ => {}
                                },
                                FormField::Labels => match key.code {
                                    KeyCode::Char('j') | KeyCode::Down => {
                                        if !form.available_labels.is_empty() {
                                            form.label_cursor = (form.label_cursor + 1)
                                                .min(form.available_labels.len() - 1);
                                        }
                                    }
                                    KeyCode::Char('k') | KeyCode::Up => {
                                        form.label_cursor = form.label_cursor.saturating_sub(1);
                                    }
                                    KeyCode::Char(' ') => {
                                        if form.label_cursor < form.selected_labels.len() {
                                            form.selected_labels[form.label_cursor] =
                                                !form.selected_labels[form.label_cursor];
                                        }
                                    }
                                    _ => {}
                                },
                            },
                        }
                    }
                }
                Mode::Board => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('j') | KeyCode::Down => app.move_down(),
                    KeyCode::Char('k') | KeyCode::Up => app.move_up(),
                    KeyCode::Char('h') | KeyCode::Left => app.move_left(),
                    KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => app.move_right(),
                    KeyCode::BackTab => app.move_left(),
                    KeyCode::Enter | KeyCode::Char('o') => app.open_detail(),
                    KeyCode::Char('c') => {
                        let labels = project.list_labels().await.unwrap_or_default();
                        app.start_create(labels);
                    }
                    KeyCode::Char('e') => {
                        if let Some(task) = app.selected_task().cloned() {
                            let labels = project.list_labels().await.unwrap_or_default();
                            app.start_edit(&task, labels);
                        }
                    }
                    _ => {}
                },
            }
        }

        // Check for app events
        while let Ok(event) = rx.try_recv() {
            match event {
                AppEvent::BoardUpdate(board) => app.update_board(board),
                AppEvent::PollError(e) => app.poll_error = Some(e),
            }
        }
    }

    Ok(())
}
