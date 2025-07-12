use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, List, ListState, Padding},
};

use crate::{models::state::State, ui::todo_item};

pub fn draw(frame: &mut Frame, rect: Rect, state: &State) {
    let Some(current_todo) = state.get_current_item() else {
        return;
    };

    let list_items = current_todo
        .items
        .iter()
        .map(|i| todo_item::create_list_item(i));

    let block = Block::new().padding(Padding::horizontal(1));

    let list = List::new(list_items)
        .block(block)
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("▶ ");

    let mut list_state = ListState::default();
    if let Some(todo) = state.get_current_item() {
        list_state.select(todo.get_current_item_index());
    }
    frame.render_stateful_widget(list, rect, &mut list_state);
}
