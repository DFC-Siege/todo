use std::{error::Error, time::Duration};

use crossterm::event::{self, Event};
use ratatui::{Terminal, prelude::Backend};

use crate::{
    handlers::{app::AppHandler, input::InputHandler},
    models::state::State,
    ui,
};

pub struct App {
    state: State,
    app_handler: AppHandler,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let app_handler = AppHandler::new()?;
        let mut state = State::new();

        app_handler.load(&mut state)?;

        Ok(Self { state, app_handler })
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        loop {
            terminal.draw(|frame| {
                ui::draw(frame, &self.state);
            })?;

            let timeout = Duration::from_millis(16); // ~60 FPS
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.is_press() {
                        InputHandler::handle_key_event(&mut self.state, key, &self.app_handler)?;
                    }
                }
            }

            if self.state.should_quit {
                break;
            }
        }

        Ok(())
    }
}
