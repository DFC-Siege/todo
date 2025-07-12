use std::io;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

use crate::state::State;

pub struct App {
    exit: bool,
    state: State,
}

impl App {
    pub fn new(state: State) -> Self {
        Self { exit: false, state }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let [border_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());

        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .fg(Color::Yellow)
            .render(border_area, frame.buffer_mut());

        List::new(
            self.state.get_items()[0]
                .get_items()
                .iter()
                .map(|i| ListItem::from(i.get_text())),
        )
        .render(inner_area, frame.buffer_mut());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::NONE, KeyCode::Char('q')) => self.exit(),
            (KeyModifiers::CONTROL, KeyCode::Char('a')) => {
                println!("test");
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
