use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, ListItem, Widget},
};

use crate::models::state::State;

pub fn draw(frame: &mut Frame, rect: Rect, state: &State) {
    let Some(current_todo) = state.get_current_item() else {
        return;
    };

    let list_items = current_todo
        .items
        .iter()
        .map(|i| ListItem::new(i.text.to_owned()));

    List::new(list_items).render(rect, frame.buffer_mut());
}
