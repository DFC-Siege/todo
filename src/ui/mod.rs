pub mod app;
pub mod theme;
pub mod todo_item;
pub mod todo_list;

use crate::models::state::State;
use ratatui::Frame;

pub fn draw(frame: &mut Frame, state: &State) {
    app::draw_main(frame, frame.area(), state);
}
