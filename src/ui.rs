use std::cmp::Ordering;

use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table},
    Frame,
};

use crate::{
    app::{StatusBar, StatusBarInput, ITEM_HEIGHT},
    reference::Reference,
    App,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(1)]).split(frame.size());

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

fn render_footer(frame: &mut Frame, app: &mut App, area: Rect) {
    let text = match &app.status_bar {
        StatusBar::Message(message) => message,
        StatusBar::Input(status_bar_input) => &status_bar_input.input,
    };

    let footer = Paragraph::new(Line::from(format!("\n   {}", text)))
        .style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg));
    frame.render_widget(footer, area);
}

pub fn move_cursor_left(status_bar_input: &StatusBarInput, cursor_position: usize) -> usize {
    let cursor_moved_left = cursor_position.saturating_sub(1);
    clamp_cursor(status_bar_input, cursor_moved_left)
}

pub fn move_cursor_right(status_bar_input: &StatusBarInput, cursor_position: usize) -> usize {
    let cursor_moved_right = cursor_position.saturating_add(1);
    //println!("position after add: {}", cursor_moved_right);
    let nexty = clamp_cursor(status_bar_input, cursor_moved_right);
    //println!("position after clamping: {}", nexty);
    nexty
}

pub fn enter_char(status_bar_input: &StatusBarInput, new_char: char) -> StatusBarInput {
    //println!("**current: {}", status_bar_input.cursor_position);
    let mut next_input: String = status_bar_input.input.clone();
    next_input.insert(status_bar_input.cursor_position, new_char);

    let next_cursor_position =
        move_cursor_right(&status_bar_input, status_bar_input.cursor_position);
    //println!("**next: {}**", next_cursor_position);
    let sbi = StatusBarInput {
        cursor_position: next_cursor_position,
        input: next_input,
    };
    sbi
}

pub fn delete_char(status_bar_input: &StatusBarInput) -> StatusBarInput {
    let cursor_is_not_leftmost = status_bar_input.cursor_position != 1;
    if cursor_is_not_leftmost {
        // Method "remove" is not used on the saved text for deleting the selected char.
        // Reason: Using remove on String works on bytes instead of the chars.
        // Using remove would require special care because of char boundaries.

        let current_index = status_bar_input.cursor_position;
        let from_left_to_current_index = current_index - 1;

        // Getting all characters before the selected character.
        let before_char_to_delete = status_bar_input
            .input
            .chars()
            .take(from_left_to_current_index);
        // Getting all characters after selected character.
        let after_char_to_delete = status_bar_input.input.chars().skip(current_index);

        // Put all characters together except the selected one.
        // By leaving the selected one out, it is forgotten and therefore deleted.
        let next_input: String = before_char_to_delete.chain(after_char_to_delete).collect();
        let next_cursor_position = move_cursor_left(&status_bar_input, current_index - 1);
        let next_status_bar_input = StatusBarInput {
            cursor_position: next_cursor_position,
            input: next_input,
        };
        next_status_bar_input
    } else {
        status_bar_input.clone()
    }
}

pub fn clamp_cursor(status_bar_input: &StatusBarInput, new_cursor_pos: usize) -> usize {
    let len = status_bar_input.input.len();
    //println!("len of status bar input: {}", len);
    new_cursor_pos.clamp(0, status_bar_input.input.len() + 1)
}
