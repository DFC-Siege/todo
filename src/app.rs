use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

use crate::{todo::Todo, todo_item::TodoItem};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    todos: Vec<Todo>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut todo = Todo::default();
        let todo_item = TodoItem::default();
        todo.add_item(todo_item);
        self.todos.push(todo);

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
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

        for todo in &self.todos {
            todo.handle_events(key_event);
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Todo's ".bold());
        let instructions = Line::from(vec![
            " new todo ".into(),
            "<^a>".blue().bold(),
            " delete todo ".into(),
            "<^d>".blue().bold(),
            " move tabs ".into(),
            "<^h-l>".blue().bold(),
            " move between items ".into(),
            "<j-k>".blue().bold(),
            " add item ".into(),
            "<a>".blue().bold(),
            " delete item ".into(),
            "<d>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .render(area, buf);
    }
}
