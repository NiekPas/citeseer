use std::cmp::Ordering;

use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Borders, Cell, HighlightSpacing, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, Table,
    },
    Frame,
};

use crate::{app::ITEM_HEIGHT, reference::Reference, App};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(frame.size());

    app.set_colors();

    render_table(frame, app, rects[0]);

    render_scrollbar(frame, app, rects[0]);

    render_footer(frame, app, rects[1], app.status_text);
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
    let items = &mut app.items;

    items.sort_by(compare_authors);

    let rows = items.iter().enumerate().map(|(i, reference)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };
        let row_style = Style::new().fg(app.colors.row_fg).bg(color);

        reference
            .as_array()
            .iter()
            .cloned()
            .map(|content| content.unwrap_or_default())
            .map(|content| Cell::from(Text::from(format!("{}", content))))
            .collect::<Row>()
            .style(row_style)
            // Using unwrap() is fine here, because ITEM_HEIGHT is a constant
            .height(ITEM_HEIGHT.try_into().unwrap())
    });

    let bar = " █ ";
    let table = Table::new(
        rows,
        [
            // + 1 is for padding.
            // key
            // This is somewhat arbitrary, but having the column be slightly less wide than to fit is fine for keys,
            // since we normally don't need to see the entire key anyway.
            Constraint::Min(app.longest_item_lens.0 - 6),
            // For the author, we use a percentage, because the longest item is going to be like 300 characters
            Constraint::Percentage(25),
            // Years are almost always 4 digits long, so setting using `Length` to 6 is fine here.
            // An exception to this is the format "1985 [1935]", which will not be entirely visible.
            Constraint::Length(6),
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

fn compare_authors(a: &Reference, b: &Reference) -> Ordering {
    a.formatted_author().cmp(&b.formatted_author())
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

fn render_footer(frame: &mut Frame, app: &mut App, area: Rect, text: &str) {
    let info_footer = Paragraph::new(Line::from(text))
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
