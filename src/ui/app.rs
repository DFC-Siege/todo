use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{models::state::State, ui::todo_list};

pub fn draw_main(frame: &mut Frame, rect: Rect, state: &State) {
    let block = Block::new()
        .title_top(Line::raw("Todo App").alignment(Alignment::Center))
        .borders(Borders::all())
        .title_bottom(Line::from(vec![Span::raw("q - quit")]).alignment(Alignment::Center));

    let inner_area = block.inner(rect);

    Paragraph::new("main")
        .block(block)
        .render(rect, frame.buffer_mut());

    todo_list::draw(frame, inner_area, state);
}
