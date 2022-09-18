#![feature(let_else)]
#![feature(absolute_path)]
#[allow(unused)]
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use hyperbol::{hyperbolic_project, PoincarePoint, DAG, DAGID};
use std::collections::HashMap;
use std::fs::{canonicalize, metadata};
use std::path::{absolute, Path, MAIN_SEPARATOR};
use std::{io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::canvas::{Canvas, Map, MapResolution},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use walkdir::WalkDir;

#[derive(Parser)]
struct Args {
    /// The root directory to render from.
    #[clap(long)]
    root: String,

    #[clap(long, default_value_t = 3)]
    depth: u32,
}

/// Represents the state of the TUI
#[derive(Debug)]
struct App {
    shift: [f64; 2],

    id_to_path: HashMap<DAGID, String>,

    path_positions: Vec<[f64; 2]>,

    buffer: String,
}

fn canonical(s: &str) -> Result<String, ()> {
    let abs = absolute(s)
        .map_err(|_| ())?
        .into_os_string()
        .into_string()
        .unwrap();
    let canon = canonicalize(abs)
        .map_err(|_| ())?
        .into_os_string()
        .into_string()
        .unwrap();
    Ok(canon)
}

fn is_dir(s: &str) -> bool {
    Path::new(s).is_dir()
}

impl App {
    pub fn new(root: &str, max_depth: usize) -> Self {
        let mut dag = DAG::new();
        let iter = WalkDir::new(root).min_depth(0).max_depth(max_depth);
        let iter = iter.into_iter().filter_map(|v| v.ok());
        let mut path_to_id = HashMap::new();
        let mut id_to_path = HashMap::new();
        for entry in iter {
            let path = entry.path().to_str().unwrap().to_string();
            let insert_id = dag.insert(path.clone());
            if entry.depth() > 0 {
                let parent_path = entry.path().parent().unwrap().to_str().unwrap().to_string();
                let parent_id = path_to_id[&parent_path];
                dag.insert_edge(parent_id, insert_id);
            }
            assert_eq!(id_to_path.insert(insert_id, path.clone()), None);
            assert_eq!(path_to_id.insert(path, insert_id), None);
        }

        let (path_positions, _) = hyperbolic_project(&dag, 0);

        let mut buffer = canonical(root).unwrap();

        if !buffer.ends_with(MAIN_SEPARATOR) && is_dir(&buffer) {
            buffer.push(MAIN_SEPARATOR)
        }

        Self {
            shift: [0.; 2],
            id_to_path,
            path_positions,
            buffer,
        }
    }

    pub fn reset_root(&mut self) -> bool {
        while self.buffer.ends_with(MAIN_SEPARATOR) && self.buffer.len() > 1 {
            self.buffer.pop();
        }

        let Ok(root) = absolute(&self.buffer) else {
            return false
        };
        let mut dag = DAG::new();
        let iter = WalkDir::new(root).min_depth(0).max_depth(3);
        let iter = iter.into_iter().filter_map(|v| v.ok());
        let mut path_to_id = HashMap::new();
        let mut id_to_path = HashMap::new();
        for entry in iter {
            let path = entry.path().to_str().unwrap().to_string();
            let insert_id = dag.insert(path.clone());
            if entry.depth() > 0 {
                let parent_path = entry.path().parent().unwrap().to_str().unwrap().to_string();
                let Some(parent_id) = path_to_id.get(&parent_path) else {
                  continue
                };
                dag.insert_edge(*parent_id, insert_id);
            }
            assert_eq!(id_to_path.insert(insert_id, path.clone()), None);
            assert_eq!(path_to_id.insert(path, insert_id), None);
        }

        if id_to_path.is_empty() {
            return false;
        }

        let (path_positions, _) = hyperbolic_project(&dag, 0);

        self.id_to_path = id_to_path;
        self.path_positions = path_positions;
        self.shift = [0.; 2];
        true
    }
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal, App::new(&args.root, 3))?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
        if event::poll(Duration::from_secs_f32(1e-3))? {
            if let Event::Key(key) = event::read()? {
                let eps = 5e-2;
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Down => {
                        app.shift[1] -= eps;
                    }
                    KeyCode::Up => {
                        app.shift[1] += eps;
                    }
                    KeyCode::Right => {
                        app.shift[0] += eps;
                    }
                    KeyCode::Left => {
                        app.shift[0] -= eps;
                    }
                    KeyCode::Backspace | KeyCode::Delete => {
                        app.buffer.pop();
                    }
                    KeyCode::Enter => {
                        if app.buffer == "" {
                            continue;
                        }
                        app.reset_root();
                        app.buffer = if let Ok(canon) = canonical(&app.buffer) {
                            canon
                        } else {
                            String::from("ERROR")
                        };
                        if is_dir(&app.buffer) && !app.buffer.ends_with(MAIN_SEPARATOR) {
                            app.buffer.push(MAIN_SEPARATOR);
                        }
                    }
                    KeyCode::Char(c) => {
                        app.buffer.push(c);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(96), Constraint::Percentage(4)].as_ref())
        .split(f.size());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("hypertree"))
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            for (i, p) in app.path_positions.iter().enumerate() {
                let new_p = PoincarePoint::from_raw(p).mobius_add(&PoincarePoint::exp(&app.shift));

                let x = new_p.0[0];
                let y = new_p.0[1];
                let Some(path) = app.id_to_path.get(&i).clone() else {
                    continue
                };
                let color = if let Ok(md) = metadata(&path) {
                    if md.file_type().is_file() {
                        Color::Green
                    } else if md.file_type().is_dir() {
                        Color::Blue
                    } else {
                        Color::Red
                    }
                } else {
                    Color::Magenta
                };
                let display = Path::new(&path).to_path_buf();
                let display: String = if let Some(file_name) = display.file_name() {
                    file_name.to_str().unwrap().to_string()
                } else {
                    display.as_os_str().to_str().unwrap().to_string()
                };
                ctx.print(x, y, Span::styled(display, Style::default().fg(color)));
            }
        })
        .x_bounds([-1., 1.])
        .y_bounds([-1., 1.]);
    f.render_widget(canvas, chunks[0]);

    let create_block = |title| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD),
        ))
    };
    let display_text = format!("Curr Directory {}", app.buffer);
    let entry = Paragraph::new(app.buffer.clone())
        //.style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block(&display_text))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(entry, chunks[1]);
}
