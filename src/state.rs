use crate::todo::Todo;

#[derive(Default)]
pub struct State {
    items: Vec<Todo>,
}

impl State {
    pub fn add_item(&mut self, todo: Todo) {
        self.items.push(todo);
    }

    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            return;
        }

        self.items.remove(index);
    }

    pub fn get_items(&self) -> &[Todo] {
        &self.items
    }
}
