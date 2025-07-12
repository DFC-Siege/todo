use crate::models::todo_item::TodoItem;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub items: Vec<TodoItem>,
    pub title: String,
    pub current_item_index: usize,
}

impl Todo {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            items: Vec::new(),
            current_item_index: 0,
        }
    }

    pub fn get_current_item(&self) -> Option<&TodoItem> {
        self.items.get(self.current_item_index)
    }

    pub fn get_current_item_mut(&mut self) -> Option<&mut TodoItem> {
        self.items.get_mut(self.current_item_index)
    }

    pub fn get_current_item_index(&self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }
        Some(self.current_item_index)
    }

    pub fn next(&mut self) -> Option<&TodoItem> {
        self.current_item_index += 1;
        if self.current_item_index >= self.items.len() {
            self.current_item_index = self.items.len() - 1;
        }

        self.items.get(self.current_item_index)
    }

    pub fn previous(&mut self) -> Option<&TodoItem> {
        if self.current_item_index != 0 {
            self.current_item_index -= 1;
        }

        self.items.get(self.current_item_index)
    }
}
