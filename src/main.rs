use core::panic;
use std::{
    fs,
    io::{self, stdout},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use reference::Reference;

use crate::parse::parse_bibtex;

extern crate shellexpand;

mod parse;
mod reference;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let path_str = "./test_bibliography_small.bib";
    if let Ok(references) = parse_file(path_str) {
        start_loop();
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn parse_file(path: &str) -> Result<Vec<Reference>, String> {
    let bibtex_string = fs::read_to_string(path).expect("Failed to open file");
    parse_bibtex(bibtex_string)
}

fn start_loop() -> () {
    if let Ok(mut terminal) = Terminal::new(CrosstermBackend::new(stdout())) {
        let mut should_quit = false;
        while !should_quit {
            terminal.draw(ui);
            if let Ok(sq) = handle_events() {
                should_quit = sq;
            } else {
                should_quit = true;
            }
        }
    }
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!")
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
        frame.size(),
    );
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
