use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    text::Line,
    widgets::{Block, List, ListItem, ListState, Padding, StatefulWidget},
};

use crate::todo_item::TodoItem;

#[derive(Debug, Clone)]
pub struct Todo {
    items: Vec<TodoItem>,
    title: String,
}

impl Todo {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            items: Vec::new(),
        }
    }
    pub fn add_item(&mut self, item: TodoItem) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) {
        if self.items.len() > index {
            self.items.remove(index);
        }
    }

    pub fn get_items(&self) -> &[TodoItem] {
        &self.items
    }
}
