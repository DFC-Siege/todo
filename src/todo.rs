use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::Widget;

use crate::todo_item::TodoItem;

#[derive(Debug, Default)]
pub struct Todo {
    items: Vec<TodoItem>,
}

impl Todo {
    pub fn add_item(&mut self, item: TodoItem) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) {
        if self.items.len() > index {
            self.items.remove(index);
        }
    }

    pub fn handle_events(&self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('a') => {}
            KeyCode::Char('d') => {}
            _ => {}
        };
    }
}

impl Widget for &Todo {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}
