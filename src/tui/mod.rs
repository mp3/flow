use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::{io, time::Duration};
use crate::db::TaskRepository;
use crate::models::{Task, Status, Priority};
use crate::context::ContextManager;

struct App {
    tasks: Vec<Task>,
    state: ListState,
}

impl App {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn run(repo: &TaskRepository) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    // Filter by current context by default
    let context_path = ContextManager::get_context()?;
    let tasks = repo.get_tasks(Some(context_path.to_str().unwrap()))?;
    let mut app = App {
        tasks,
        state: ListState::default(),
    };
    if !app.tasks.is_empty() {
        app.state.select(Some(0));
    }

    let res = run_app(&mut terminal, &mut app, repo);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, repo: &TaskRepository) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if let Some(i) = app.state.selected() {
                            if let Some(task) = app.tasks.get(i) {
                                // Toggle status (Todo -> Done -> Todo)
                                // For now just mark done
                                if task.status != Status::Done {
                                    repo.complete_task(task.id.unwrap())?;
                                } else {
                                    // Re-open? We didn't implement re-open in DB yet.
                                    // Just ignore for now.
                                }
                                // Reload tasks
                                let context_path = ContextManager::get_context()?;
                                app.tasks = repo.get_tasks(Some(context_path.to_str().unwrap()))?;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.size());

    let tasks: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|t| {
            let (icon, style) = if t.status == Status::Done {
                ("✔", Style::default().fg(Color::Green))
            } else {
                ("☐", Style::default())
            };
            
            let priority_color = match t.priority {
                Priority::High | Priority::Critical => Color::Red,
                Priority::Medium => Color::Yellow,
                Priority::Low => Color::Blue,
            };

            let content = Line::from(vec![
                Span::styled(format!("{} ", icon), style),
                Span::styled(format!("{} ", t.title), if t.status == Status::Done { Style::default().add_modifier(Modifier::CROSSED_OUT) } else { Style::default() }),
                Span::styled(format!("[{}]", t.priority), Style::default().fg(priority_color)),
            ]);
            ListItem::new(content)
        })
        .collect();

    let tasks_list = List::new(tasks)
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::DarkGray))
        .highlight_symbol("> ");

    f.render_stateful_widget(tasks_list, chunks[0], &mut app.state);

    let help = Paragraph::new("j/k: navigate | space/enter: toggle | q: quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[1]);
}
