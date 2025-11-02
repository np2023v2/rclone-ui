use crate::rclone::{FileItem, RcloneClient, Remote};
use crate::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use tokio::time::Duration;

/// Application state
pub struct App {
    client: RcloneClient,
    view: ViewMode,
    remotes: Vec<Remote>,
    files: Vec<FileItem>,
    selected: ListState,
    status_message: String,
    current_remote: Option<String>,
    current_path: String,
}

#[derive(Debug, Clone)]
pub enum ViewMode {
    Remotes,
    Files,
}

impl App {
    pub fn new(client: RcloneClient) -> Self {
        let mut selected = ListState::default();
        selected.select(Some(0));

        Self {
            client,
            view: ViewMode::Remotes,
            remotes: Vec::new(),
            files: Vec::new(),
            selected,
            status_message: "Welcome to Rclone UI! Press 'h' for help.".to_string(),
            current_remote: None,
            current_path: String::new(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // Check if rclone is available
        match self.client.check_rclone() {
            Ok(version) => {
                self.status_message = format!("Rclone detected: {}", version);
            }
            Err(e) => {
                self.status_message = format!("Error: rclone not found - {}", e);
            }
        }

        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    async fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        self.refresh_remotes().await?;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && self.handle_input(key.code).await? {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_input(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('h') => {
                self.status_message =
                    "Commands: q=quit, Enter=open, b=back, r=refresh, ↑↓=navigate".to_string();
            }
            KeyCode::Char('r') => match self.view {
                ViewMode::Remotes => {
                    self.refresh_remotes().await?;
                    self.status_message = "Remotes refreshed".to_string();
                }
                ViewMode::Files => {
                    if let Some(remote) = self.current_remote.clone() {
                        let path = self.current_path.clone();
                        self.refresh_files(&remote, &path).await?;
                        self.status_message = "Files refreshed".to_string();
                    }
                }
            },
            KeyCode::Char('b') => {
                match self.view {
                    ViewMode::Remotes => {
                        // Already at top level
                        self.status_message = "Already at remotes view".to_string();
                    }
                    ViewMode::Files => {
                        if self.current_path.is_empty() {
                            // Go back to remotes
                            self.view = ViewMode::Remotes;
                            self.current_remote = None;
                            self.current_path.clear();
                            self.refresh_remotes().await?;
                            self.status_message = "Back to remotes".to_string();
                        } else {
                            // Go up one directory
                            let parts: Vec<&str> = self.current_path.rsplitn(2, '/').collect();
                            if parts.len() > 1 {
                                self.current_path = parts[1].to_string();
                            } else {
                                self.current_path.clear();
                            }
                            if let Some(remote) = self.current_remote.clone() {
                                let path = self.current_path.clone();
                                self.refresh_files(&remote, &path).await?;
                            }
                            self.status_message = "Went up one directory".to_string();
                        }
                    }
                }
            }
            KeyCode::Enter => {
                match self.view {
                    ViewMode::Remotes => {
                        if let Some(index) = self.selected.selected() {
                            if index < self.remotes.len() {
                                let remote = self.remotes[index].clone();
                                self.current_remote = Some(remote.name.clone());
                                self.current_path.clear();
                                self.view = ViewMode::Files;
                                self.refresh_files(&remote.name, "").await?;
                                self.status_message = format!("Opened remote: {}", remote.name);
                            }
                        }
                    }
                    ViewMode::Files => {
                        if let Some(index) = self.selected.selected() {
                            if index < self.files.len() {
                                let file = self.files[index].clone();
                                if file.is_dir {
                                    // Navigate into directory
                                    self.current_path = if self.current_path.is_empty() {
                                        file.path.clone()
                                    } else {
                                        format!("{}/{}", self.current_path, file.path)
                                    };
                                    if let Some(remote) = self.current_remote.clone() {
                                        let path = self.current_path.clone();
                                        self.refresh_files(&remote, &path).await?;
                                    }
                                    self.status_message =
                                        format!("Opened directory: {}", file.name);
                                } else {
                                    self.status_message = format!(
                                        "File: {} ({})",
                                        file.name,
                                        self.format_size(file.size)
                                    );
                                }
                            }
                        }
                    }
                }
            }
            KeyCode::Down => {
                let len = match self.view {
                    ViewMode::Remotes => self.remotes.len(),
                    ViewMode::Files => self.files.len(),
                };

                let i = match self.selected.selected() {
                    Some(i) => {
                        if i >= len.saturating_sub(1) {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                self.selected.select(Some(i));
            }
            KeyCode::Up => {
                let len = match self.view {
                    ViewMode::Remotes => self.remotes.len(),
                    ViewMode::Files => self.files.len(),
                };

                let i = match self.selected.selected() {
                    Some(i) => {
                        if i == 0 {
                            len.saturating_sub(1)
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                self.selected.select(Some(i));
            }
            _ => {}
        }
        Ok(false)
    }

    async fn refresh_remotes(&mut self) -> Result<()> {
        self.remotes = match self.client.list_remotes().await {
            Ok(remotes) => remotes,
            Err(e) => {
                self.status_message = format!("Error loading remotes: {}", e);
                Vec::new()
            }
        };

        // Adjust selection if needed
        if self.remotes.is_empty() {
            self.selected.select(None);
        } else if let Some(selected) = self.selected.selected() {
            if selected >= self.remotes.len() {
                self.selected.select(Some(self.remotes.len() - 1));
            }
        } else {
            self.selected.select(Some(0));
        }

        Ok(())
    }

    async fn refresh_files(&mut self, remote: &str, path: &str) -> Result<()> {
        self.files = match self.client.list_files(remote, path).await {
            Ok(files) => files,
            Err(e) => {
                self.status_message = format!("Error loading files: {}", e);
                Vec::new()
            }
        };

        // Adjust selection if needed
        if self.files.is_empty() {
            self.selected.select(None);
        } else if let Some(selected) = self.selected.selected() {
            if selected >= self.files.len() {
                self.selected.select(Some(self.files.len() - 1));
            }
        } else {
            self.selected.select(Some(0));
        }

        Ok(())
    }

    fn format_size(&self, size: i64) -> String {
        const KB: i64 = 1024;
        const MB: i64 = KB * 1024;
        const GB: i64 = MB * 1024;

        if size >= GB {
            format!("{:.2} GB", size as f64 / GB as f64)
        } else if size >= MB {
            format!("{:.2} MB", size as f64 / MB as f64)
        } else if size >= KB {
            format!("{:.2} KB", size as f64 / KB as f64)
        } else {
            format!("{} B", size)
        }
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());

        // Title
        let title_text = match self.view {
            ViewMode::Remotes => "☁️  Rclone UI - Remotes",
            ViewMode::Files => {
                if let Some(ref remote) = self.current_remote {
                    if self.current_path.is_empty() {
                        return f.render_widget(
                            Paragraph::new(format!("☁️  Rclone UI - {}", remote))
                                .style(Style::default().fg(Color::Cyan))
                                .alignment(Alignment::Center)
                                .block(Block::default().borders(Borders::ALL)),
                            chunks[0],
                        );
                    } else {
                        return f.render_widget(
                            Paragraph::new(format!(
                                "☁️  Rclone UI - {}:{}",
                                remote, self.current_path
                            ))
                            .style(Style::default().fg(Color::Cyan))
                            .alignment(Alignment::Center)
                            .block(Block::default().borders(Borders::ALL)),
                            chunks[0],
                        );
                    }
                }
                "☁️  Rclone UI - Files"
            }
        };

        let title = Paragraph::new(title_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Main content
        match self.view {
            ViewMode::Remotes => {
                let items: Vec<ListItem> = self
                    .remotes
                    .iter()
                    .map(|remote| {
                        let content = format!("📦 {}", remote.name);
                        ListItem::new(content).style(Style::default().fg(Color::White))
                    })
                    .collect();

                let list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(format!("Remotes ({})", self.remotes.len())),
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                f.render_stateful_widget(list, chunks[1], &mut self.selected);
            }
            ViewMode::Files => {
                let items: Vec<ListItem> = self
                    .files
                    .iter()
                    .map(|file| {
                        let icon = if file.is_dir { "📁" } else { "📄" };
                        let size_str = if file.is_dir {
                            String::new()
                        } else {
                            format!(" ({})", self.format_size(file.size))
                        };
                        let content = format!("{} {}{}", icon, file.name, size_str);
                        ListItem::new(content).style(Style::default().fg(Color::White))
                    })
                    .collect();

                let list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(format!("Files ({})", self.files.len())),
                    )
                    .highlight_style(
                        Style::default()
                            .bg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                f.render_stateful_widget(list, chunks[1], &mut self.selected);
            }
        }

        // Status bar
        let status = Paragraph::new(self.status_message.clone())
            .style(Style::default())
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Status"));

        f.render_widget(status, chunks[2]);
    }
}
