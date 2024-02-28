// To do:
// - Make field parsing case-insensitive

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
use ui::{delete_char, ui};

use crate::{
    app::{StatusBar, StatusBarInput},
    ui::enter_char,
};

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
        // TODO make input a general widget, instead of putting it in ui
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.status_bar {
                    // If the status bar is displaying a message, we are in the state where
                    // we should handle keypresses as key commands (h, j, k, q, etc.)
                    StatusBar::Message(_) => {
                        if handle_keyboard_command(&mut app, key.code) {
                            return Ok(());
                        }
                    }
                    // Otherwise, we should handle keypresses as status bar input
                    StatusBar::Input(ref status_bar_input) => {
                        app.status_bar = handle_status_bar_input(status_bar_input, key)
                    }
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

fn handle_keyboard_command(app: &mut App, key_code: KeyCode) -> bool {
    use KeyCode::*;
    if key_code == Char('q') {
        return true;
    }
    match key_code {
        Char('j') | Down => app.select_next(),
        Char('k') | Up => app.select_previous(),
        Char('l') | Right => app.next_color(),
        Char('h') | Left => app.previous_color(),
        Char('y') => match app.yank() {
            Some(reference) => {
                app.status_bar = StatusBar::Message(format!(
                    "Copied {} to the clipboard as BibTeX.",
                    reference.key
                ));
            }
            None => app.status_bar = StatusBar::Message(String::from("Yank failed.")),
        },
        Char('/') => {
            let status_bar_input = StatusBarInput {
                input: String::from("/"),
                cursor_position: 1, // The starting position is 1 because the search field always contains a '/'
            };

            app.status_bar = StatusBar::Input(status_bar_input);
        }
        _ => {}
    }
    false
}

fn handle_status_bar_input(status_bar_input: &StatusBarInput, key: event::KeyEvent) -> StatusBar {
    use KeyCode::*;
    match key.code {
        // Backspace removes characters
        Backspace => {
            let sb = delete_char(status_bar_input);
            StatusBar::Input(sb)
        }
        // ESC resets the status bar to displaying a (blank) message
        Esc => StatusBar::Message(String::default()),
        // Any other char should be entered into the input field
        Char(c) => StatusBar::Input(enter_char(status_bar_input, c)),
        // No-op
        _ => StatusBar::Input(status_bar_input.clone()),
    }
}
