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
    widgets::{Block, Paragraph},
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
        start_loop(&references);
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn parse_file(path: &str) -> Result<Vec<Reference>, String> {
    let bibtex_string = fs::read_to_string(path).expect("Failed to open file");
    parse_bibtex(bibtex_string)
}

fn start_loop(references: &Vec<Reference>) {
    if let Ok(mut terminal) = Terminal::new(CrosstermBackend::new(stdout())) {
        loop {
            match terminal.draw(|frame| ui(frame, references)) {
                Err(_) => break,
                Ok(_) => (),
            }
            match handle_events() {
                Ok(true) => break,
                _ => (),
            }
        }
    }
}

fn ui(frame: &mut Frame, references: &Vec<Reference>) {
    let paragraph_text = references
        .first()
        .expect("no references found")
        .key
        .as_str();
    frame.render_widget(
        Paragraph::new(paragraph_text).block(Block::default().title("Greeting")),
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
