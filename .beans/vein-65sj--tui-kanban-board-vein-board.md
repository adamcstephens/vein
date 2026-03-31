---
# vein-65sj
title: TUI kanban board (vein board)
status: in-progress
type: feature
priority: normal
created_at: 2026-03-31T02:40:00Z
updated_at: 2026-03-31T02:54:33Z
---

Read-only 3-panel TUI kanban board that polls Vikunja and displays Ready | In Progress | Done columns. Uses ratatui + crossterm. Single API call per poll cycle (5s interval). Navigation with arrow keys / j/k/h/l/tab. No move/open interactivity in first pass.


## Implementation Plan

### 1. Add `list_board()` to `ProjectClient`

Add a `BoardState` struct and `list_board()` method to `project.rs`:

```rust
pub struct BoardState {
    pub ready: Vec<Task>,
    pub in_progress: Vec<Task>,
    pub done: Vec<Task>,
}
```

- Calls `client.list_buckets(project_id, view_id)` — single HTTP request
- Extracts tasks from each bucket by matching config bucket IDs
- Applies `is_blocked()` filter to ready column
- Sorts each column by position
- Unit tests with `MockClient`

### 2. Add ratatui + crossterm dependencies

- `ratatui` (latest)
- `crossterm` (latest)
- Ask permission before adding

### 3. Create `src/board.rs` — TUI module

**App state:**
- `BoardState` holding the three task lists
- `selected_column: usize` (0=ready, 1=in_progress, 2=done)
- `selected_index: [usize; 3]` per-column cursor positions
- `last_poll: Instant` and `poll_error: Option<String>` for status display

**Event loop (tokio + crossterm):**
- `tokio::spawn` background task polls `list_board()` every 5s, sends `BoardState` over `mpsc`
- Main loop `select!`s between:
  - Crossterm key events (q=quit, j/k=up/down, h/l/tab=switch column)
  - Channel updates (new board state)
- Re-render on any event

**Rendering (ratatui):**
- `Layout::horizontal` split into 3 equal chunks
- Each panel: `Block` with title ("Ready", "In Progress", "Done") + task count
- Each task rendered as `ListItem`: `"{display_id}: {title}"` with priority/label indicators
- Highlight selected item in active column
- Status bar at bottom: last refresh time, error state, keybind hints

### 4. Add `Board` command to CLI

- New `Command::Board` variant in `cli.rs` (no arguments needed)
- Handle in `main.rs`: build `ProjectClient`, call `board::run(project_client)`
- `board::run()` sets up terminal (enter alternate screen, enable raw mode), runs event loop, restores terminal on exit

### 5. Testing strategy

- Unit test `list_board()` with mock client (bucket splitting, blocked filtering, sorting)
- Manual TUI testing against dev Vikunja

### Follow-up tickets (after this ships)

- Move tasks between columns (claim/complete via keybinds)
- Open task detail in modal overlay
- Configurable poll interval
