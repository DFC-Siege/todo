use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    handlers::app::AppHandler,
    models::state::{AppState, Popup, State},
};

pub struct InputHandler;

impl InputHandler {
    pub fn handle_key_event(
        state: &mut State,
        key: KeyEvent,
        app_handler: &AppHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match state.app_state {
            AppState::Normal => Self::handle_normal_state(state, key, app_handler),
            AppState::Writing => Self::handle_writing_state(state, key, app_handler),
        }?;

        Ok(())
    }

    fn handle_normal_state(
        state: &mut State,
        key: KeyEvent,
        app_handler: &AppHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('q')) => {
                state.should_quit = true;
            }
            (KeyModifiers::NONE, KeyCode::Char('h')) => {
                state.previous();
            }
            (KeyModifiers::NONE, KeyCode::Char('l')) => {
                state.next();
            }
            (KeyModifiers::NONE, KeyCode::Char('j')) => {
                if let Some(todo) = state.get_current_item_mut() {
                    todo.next();
                }
            }
            (KeyModifiers::NONE, KeyCode::Char('k')) => {
                if let Some(todo) = state.get_current_item_mut() {
                    todo.previous();
                }
            }
            (KeyModifiers::NONE, KeyCode::Enter) => {
                if let Some(todo) = state.get_current_item_mut() {
                    if let Some(todo_item) = todo.get_current_item_mut() {
                        todo_item.toggle();
                    }
                }
            }
            (KeyModifiers::CONTROL, KeyCode::Char('n')) => state.open_popup(Popup::Create),
            _ => {}
        };
        Ok(())
    }

    fn handle_writing_state(
        state: &mut State,
        key: KeyEvent,
        app_handler: &AppHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Esc) => state.open_popup(Popup::None),
            (KeyModifiers::NONE, KeyCode::Char(c)) => state.input.handle_input(c),
            (KeyModifiers::NONE, KeyCode::Left) => state.input.left(),
            (KeyModifiers::NONE, KeyCode::Right) => state.input.right(),
            (KeyModifiers::NONE, KeyCode::Backspace) => state.input.backspace(),
            (KeyModifiers::NONE, KeyCode::Backspace) => state.input.delete(),
            _ => {}
        };
        Ok(())
    }
}
