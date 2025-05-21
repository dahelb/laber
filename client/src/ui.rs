use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100), Constraint::Min(3)])
        .split(frame.area());

    let title = Line::from("Laber").bold();

    let block = Block::bordered().title(title).border_set(border::THICK);
    let messages_widget = Paragraph::new(Line::from("Hello, world"))
        .centered()
        .block(block);

    frame.render_widget(messages_widget, chunks[0]);

    let input_block = Block::bordered().border_set(border::DOUBLE);
    let input_widget = Paragraph::new(app.input.clone()).block(input_block);

    frame.render_widget(input_widget, chunks[1]);
}
