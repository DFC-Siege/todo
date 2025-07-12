use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{handlers::app::AppHandler, models::state::State};

pub struct InputHandler;

impl InputHandler {
    pub fn handle_key_event(
        state: &mut State,
        key: KeyEvent,
        app_handler: &AppHandler,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('q')) => {
                state.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }
}
