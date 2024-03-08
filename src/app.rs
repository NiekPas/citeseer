use ratatui::{
    style::{palette::tailwind, Color},
    widgets::{ScrollbarState, TableState},
};
use unicode_width::UnicodeWidthStr;

use crate::reference::{FieldType, Reference};

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
    pub search_result_fg: Color,
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
            search_result_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

#[derive(Clone)]
pub struct StatusBarInput {
    // The current input
    pub input: String,
    // The position of the cursor in the input
    pub cursor_position: usize,
}

#[derive(Clone)]
pub enum StatusBar {
    // Displaying a message
    Message(String),
    // Receiving user input
    Input(StatusBarInput),
}

pub struct App {
    pub state: TableState,
    pub items: Vec<Reference>,
    pub longest_item_lens: (u16, u16, u16, u16, u16), // order is (key, author, year, title)
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub visible_headers: Vec<FieldType>,
    pub colors: TableColors,
    pub color_index: usize,
    pub status_bar: StatusBar,
    pub search_results: Vec<Reference>,
}

impl App {
    pub fn new(references: Vec<Reference>) -> App {
        let longest_item_lens = constraint_len_calculator(&references);
        let visible_headers = vec![Key, Type, Author, Year, Title];
        use FieldType::*;

        App {
            state: TableState::default().with_selected(0),
            longest_item_lens,
            vertical_scroll_state: ScrollbarState::new((references.len() - 1) * ITEM_HEIGHT),
            horizontal_scroll_state: ScrollbarState::new(visible_headers.len()),
            visible_headers,
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: references,
            status_bar: StatusBar::Message(String::default()),
            search_results: Vec::new(),
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
        self.vertical_scroll_state = self.vertical_scroll_state.position(i * ITEM_HEIGHT);
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
        self.vertical_scroll_state = self.vertical_scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn select_next_column(&mut self) {
        self.horizontal_scroll_state.next();
    }

    pub fn select_previous_column(&mut self) {
        self.horizontal_scroll_state.prev();
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index])
    }

    pub fn yank(&self) -> Option<&Reference> {
        let currently_selected_index = self.state.selected()?;
        let currently_selected_reference: &Reference = self.items.get(currently_selected_index)?;
        let reference_bibtex = currently_selected_reference.to_bibtex();
        if let Ok(_) = cli_clipboard::set_contents(reference_bibtex) {
            Some(currently_selected_reference)
        } else {
            None
        }
    }

    // TODO write tests for this function
    pub fn search(&mut self) {
        fn reference_contains(reference: &Reference, pattern: &str) -> bool {
            let title_contains_search_string: bool = match reference.title() {
                Some(title) => title.to_lowercase().contains(&pattern.to_lowercase()),
                None => false,
            };
            let fields_contain_search_string: bool = reference
                .fields
                .iter()
                .any(|(_field, value)| value.to_lowercase().contains(&pattern.to_lowercase()));

            title_contains_search_string || fields_contain_search_string
        }

        let search_results: Vec<Reference> = match &self.status_bar {
            StatusBar::Message(_) => Vec::new(),
            StatusBar::Input(status_bar_input) => {
                let search_value = status_bar_input.input.trim_start_matches('/');

                self.items
                    .iter()
                    .filter(|reference| reference_contains(*reference, search_value))
                    .cloned()
                    .collect()
            }
        };

        self.search_results = search_results;
    }
}

pub fn constraint_len_calculator(items: &[Reference]) -> (u16, u16, u16, u16, u16) {
    fn make_lines(title: Option<String>) -> Vec<String> {
        match title {
            Some(string) => string.lines().map(|s| s.to_owned()).collect(),
            None => vec![],
        }
    }

    let key_len = items
        .iter()
        .map(Reference::key)
        .flat_map(|key| match key {
            Some(s) => s.lines(),
            _ => "".lines(),
        })
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let reference_type_len = items
        .iter()
        .map(Reference::reference_type)
        .flat_map(|reference_type| match reference_type {
            Some(s) => s.lines(),
            _ => "".lines(),
        })
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let author_len = items
        .iter()
        .map(Reference::formatted_author)
        .flat_map(|author| make_lines(author))
        .map(|s| UnicodeWidthStr::width(&s as &str))
        .max()
        .unwrap_or(0);

    let year_len = items
        .iter()
        .map(Reference::year)
        .flat_map(|year| match year {
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
        reference_type_len as u16,
        author_len as u16,
        year_len as u16,
        title_len as u16,
    )
}
