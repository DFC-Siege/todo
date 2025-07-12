use crate::models::todo_item::TodoItem;

#[derive(Debug, Clone)]
pub struct Todo {
    pub items: Vec<TodoItem>,
    pub title: String,
}

impl Todo {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            items: Vec::new(),
        }
    }
}
