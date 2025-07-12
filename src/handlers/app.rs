use crate::models::{state::State, todo::Todo, todo_item::TodoItem};

pub struct AppHandler;

impl AppHandler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }

    pub fn initialize(&self, state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo = Todo::new("todo 1");
        let mut todo_item = TodoItem::new("todo item 1");
        todo_item.toggle();
        todo.items.push(todo_item);
        todo.items.push(TodoItem::new("todo item 2"));
        state.items.push(todo);
        let mut todo = Todo::new("todo 2");
        todo.items.push(TodoItem::new("todo item 1"));
        todo.items.push(TodoItem::new("todo item 2"));
        state.items.push(todo);
        Ok(())
    }
}
