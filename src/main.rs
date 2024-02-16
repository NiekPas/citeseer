mod parse;
mod reference;

use std::{error::Error, fs, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use parse::parse_bibtex;
use ratatui::{prelude::*, widgets::*};
use reference::Reference;
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];
const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move down | (→) next color | (←) previous color";

const ITEM_HEIGHT: usize = 4;

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
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

struct App {
    state: TableState,
    items: Vec<Reference>,
    longest_item_lens: (u16, u16, u16, u16), // order is (key, author, year, title)
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

impl App {
    fn new() -> App {
        let path_str = "./test_bibliography_small.bib";
        let references = parse_file(path_str).expect("Failed to parse file");

        App {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&references),
            scroll_state: ScrollbarState::new((references.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: references,
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
}

fn parse_file(path: &str) -> Result<Vec<Reference>, String> {
    let bibtex_string = fs::read_to_string(path).expect("Failed to open file");
    parse_bibtex(bibtex_string)
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                use KeyCode::*;
                match key.code {
                    Char('q') | Esc => return Ok(()),
                    Char('j') | Down => app.select_next(),
                    Char('k') | Up => app.select_previous(),
                    Char('l') | Right => app.next_color(),
                    Char('h') | Left => app.previous_color(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(frame.size());

    app.set_colors();

    render_table(frame, app, rects[0]);

    render_scrollbar(frame, app, rects[0]);

    render_footer(frame, app, rects[1]);
}

fn render_table(frame: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let header = ["Key", "Authors", "Year", "Title"]
        .iter()
        .cloned()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);
    let rows = app.items.iter().enumerate().map(|(i, reference)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };

        reference
            .as_array()
            .iter()
            .cloned()
            .map(|content| content.unwrap_or_default())
            .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
            .collect::<Row>()
            .style(Style::new().fg(app.colors.row_fg).bg(color))
            .height(4)
    });

    let bar = " █ ";
    let table = Table::new(
        rows,
        [
            // + 1 is for padding.
            // key
            Constraint::Min(app.longest_item_lens.0 + 1),
            // author
            Constraint::Min((app.longest_item_lens.1) + 1),
            // Years are pretty much always 4 digits long, so setting using `Length` is fine here
            Constraint::Length(app.longest_item_lens.2 + 1),
            // Let title take up the rest of the space
            Constraint::Fill(1),
        ],
    )
    .header(header)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.colors.buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(table, area, &mut app.state);
}

fn constraint_len_calculator(items: &[Reference]) -> (u16, u16, u16, u16) {
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
            _ => "".lines(), // TODO I don't know if this is the best approach
        })
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let title_len = items
        .iter()
        .map(Reference::title)
        .flat_map(|title| match title {
            Some(s) => s.lines(),
            _ => "".lines(), // TODO I don't know if this is the best approach
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

fn render_scrollbar(frame: &mut Frame, app: &mut App, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

fn render_footer(frame: &mut Frame, app: &mut App, area: Rect) {
    let info_footer = Paragraph::new(Line::from(INFO_TEXT))
        .style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg))
        .centered()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(app.colors.footer_border_color))
                .border_type(BorderType::Double),
        );
    frame.render_widget(info_footer, area);
}

// #[cfg(test)]
// mod tests {
//     use crate::Reference;

//     #[test]
//     fn constraint_len_calculator() {
//         let test_data = vec![
//             Reference {
//                 name: "Emirhan Tala".to_string(),
//                 address: "Cambridgelaan 6XX\n3584 XX Utrecht".to_string(),
//                 title: "tala.emirhan@gmail.com".to_string(),
//             },
//             Reference {
//                 name: "thistextis26characterslong".to_string(),
//                 address: "this line is 31 characters long\nbottom line is 33 characters long"
//                     .to_string(),
//                 title: "thisemailis40caharacterslong@ratatui.com".to_string(),
//             },
//         ];
//         let (longest_name_len, longest_address_len, longest_email_len) =
//             crate::constraint_len_calculator(&test_data);

//         assert_eq!(26, longest_name_len);
//         assert_eq!(33, longest_address_len);
//         assert_eq!(40, longest_email_len);
//     }
// }
