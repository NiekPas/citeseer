use std::fs;

use ratatui::{
    style::{palette::tailwind, Color},
    widgets::{ScrollbarState, TableState},
};
use unicode_width::UnicodeWidthStr;

use crate::{parse::parse_bibtex, reference::Reference};

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

pub const ITEM_HEIGHT: usize = 1;

pub struct TableColors {
    pub buffer_bg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_style_fg: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
    pub footer_border_color: Color,
}

impl TableColors {
    fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

pub struct App<'a> {
    pub state: TableState,
    pub items: Vec<Reference>,
    pub longest_item_lens: (u16, u16, u16, u16), // order is (key, author, year, title)
    pub scroll_state: ScrollbarState,
    pub colors: TableColors,
    pub color_index: usize,
    pub status_text: &'a str,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let path_str = "./test_bibliography.bib";
        let references = parse_file(path_str).expect("Failed to parse file");

        App {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&references),
            scroll_state: ScrollbarState::new((references.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: references,
            status_text: "",
        }
    }

    pub fn select_next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn select_previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index])
    }

    pub fn yank(&self) -> Option<()> {
        let currently_selected_index = self.state.selected()?;
        let currently_selected_reference: &Reference = self.items.get(currently_selected_index)?;
        let reference_bibtex = currently_selected_reference.to_bibtex();
        cli_clipboard::set_contents(reference_bibtex).ok()
    }
}

fn parse_file(path: &str) -> Result<Vec<Reference>, String> {
    let bibtex_string = fs::read_to_string(path).expect("Failed to open file");
    parse_bibtex(bibtex_string)
}

pub fn constraint_len_calculator(items: &[Reference]) -> (u16, u16, u16, u16) {
    fn make_lines(title: Option<String>) -> Vec<String> {
        match title {
            Some(string) => string.lines().map(|s| s.to_owned()).collect(),
            None => vec![],
        }
    }

    let key_len = items
        .iter()
        .map(Reference::key)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let author_len = items
        .iter()
        .map(Reference::formatted_author)
        .flat_map(|title| make_lines(title))
        .map(|s| UnicodeWidthStr::width(&s as &str))
        .max()
        .unwrap_or(0);

    let year_len = items
        .iter()
        .map(Reference::year)
        .flat_map(|title| match title {
            Some(s) => s.lines(),
            _ => "".lines(),
        })
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let title_len = items
        .iter()
        .map(Reference::title)
        .flat_map(|title| match title {
            Some(s) => s.lines(),
            _ => "".lines(),
        })
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    (
        key_len as u16,
        author_len as u16,
        year_len as u16,
        title_len as u16,
    )
}
