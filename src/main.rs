// To do:
// - Last name heuristic: if full name is of the form "John Doe", assume first-last order
// - Make field parsing case-insensitive
// - Add a search bar
// - Write tests

mod app;
mod parse;
mod reference;
mod ui;

use std::{error::Error, fs, io, path::PathBuf, process::exit};

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use parse::parse_bibtex;
use ratatui::prelude::*;
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    // 1. Try to get path from args
    // 2. Try to get path from ~/.citeseer
    // 3. Exit with message
    let path_str: String = get_path_str(&args).unwrap_or_else(|| {
        get_last_bibliography_file().unwrap_or_else(|| {
            println!("Please provide a path to a .bib file.");
            exit(1);
        })
    });

    set_last_bibliography_file(&path_str);

    let bibtex_string =
        fs::read_to_string(&path_str).expect(format!("Failed to open file: {}", path_str).as_str());

    let references =
        parse_bibtex(bibtex_string).expect(format!("Failed to parse file: {}", path_str).as_str());

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(references);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn get_path_str(args: &Vec<String>) -> Option<String> {
    if args.len() == 1 {
        return None;
    }
    Some(args[1].clone())
}

fn get_last_bibliography_file() -> Option<String> {
    let path = settings_path()?;
    let contents = fs::read_to_string(path).ok()?;
    Some(contents)
}

fn set_last_bibliography_file(bibliography_path: &String) -> Option<()> {
    let path = settings_path()?;
    fs::write(path, bibliography_path).ok()
}

fn run_app<'a, B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                // Reset status text on any key press
                app.status_text = String::default();
                use KeyCode::*;
                match key.code {
                    Char('q') | Esc => return Ok(()),
                    Char('j') | Down => app.select_next(),
                    Char('k') | Up => app.select_previous(),
                    Char('l') | Right => app.next_color(),
                    Char('h') | Left => app.previous_color(),
                    Char('y') => match app.yank() {
                        Some(reference) => {
                            app.status_text =
                                format!("Copied {} to the clipboard as BibTeX.", reference.key);
                        }
                        None => app.status_text = String::from("Yank failed."),
                    },
                    _ => {}
                }
            }
        }
    }
}

fn settings_path() -> Option<PathBuf> {
    let mut path = dirs::home_dir()?;
    path.push(".citeseer");
    path.push("settings");
    return Some(path);
}
