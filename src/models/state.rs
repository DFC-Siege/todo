use crate::models::todo::Todo;

#[derive(Default)]
pub struct State {
    pub items: Vec<Todo>,
    current_item_index: usize,
    pub should_quit: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current_item_index: 0,
            should_quit: false,
        }
    }
    pub fn get_current_item(&self) -> Option<&Todo> {
        self.items.get(self.current_item_index)
    }

    pub fn get_current_item_index(&self) -> usize {
        self.current_item_index
    }

    pub fn next(&mut self) -> Option<&Todo> {
        self.current_item_index += 1;
        if self.current_item_index >= self.items.len() {
            self.current_item_index = self.items.len() - 1;
        }

        self.items.get(self.current_item_index)
    }

    pub fn previous(&mut self) -> Option<&Todo> {
        self.current_item_index -= 1;
        if self.current_item_index <= 0 {
            self.current_item_index = 0;
        }

        self.items.get(self.current_item_index)
    }
}
