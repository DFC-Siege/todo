use crate::models::{input::Input, todo::Todo};

pub enum Popup {
    None,
    Create,
    Delete,
    Rename,
}

pub enum AppState {
    Normal,
    Writing,
}

pub struct State {
    pub items: Vec<Todo>,
    current_item_index: usize,
    pub should_quit: bool,
    pub popup: Popup,
    pub app_state: AppState,
    pub input: Input,
}

impl State {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current_item_index: 0,
            should_quit: false,
            popup: Popup::None,
            app_state: AppState::Normal,
            input: Input::default(),
        }
    }

    pub fn open_popup(&mut self, popup: Popup) {
        self.popup = popup;
        match self.popup {
            Popup::None => self.app_state = AppState::Normal,
            _ => self.app_state = AppState::Writing,
        };
    }

    pub fn get_current_item(&self) -> Option<&Todo> {
        self.items.get(self.current_item_index)
    }

    pub fn get_current_item_mut(&mut self) -> Option<&mut Todo> {
        self.items.get_mut(self.current_item_index)
    }

    pub fn get_current_item_index(&self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }
        Some(self.current_item_index)
    }

    pub fn next(&mut self) -> Option<&Todo> {
        self.current_item_index += 1;
        if self.current_item_index >= self.items.len() {
            self.current_item_index = self.items.len() - 1;
        }

        self.items.get(self.current_item_index)
    }

    pub fn previous(&mut self) -> Option<&Todo> {
        if self.current_item_index != 0 {
            self.current_item_index -= 1;
        }
        self.items.get(self.current_item_index)
    }
}
