use crate::models::{input::Input, todo::Todo, todo_item::TodoItem};

pub enum Popup {
    None,
    CreateTodo,
    RenameTodo,
    CreateTodoItem,
    RenameTodoItem,
    DeleteTodo,
    DeleteTodoItem,
}

pub enum AppState {
    Normal,
    Writing,
    Confirm,
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
            Popup::CreateTodo => self.app_state = AppState::Writing,
            Popup::CreateTodoItem => self.app_state = AppState::Writing,
            Popup::RenameTodo => {
                if let Some(todo) = self.get_current_item_mut() {
                    self.input.value = todo.title.to_owned();
                    self.input.cursor_to_end();
                    self.app_state = AppState::Writing
                }
            }
            Popup::RenameTodoItem => {
                if let Some(todo) = self.get_current_item_mut() {
                    if let Some(todo_item) = todo.get_current_item_mut() {
                        self.input.value = todo_item.text.to_owned();
                        self.input.cursor_to_end();
                        self.app_state = AppState::Writing
                    }
                }
            }
            Popup::DeleteTodo => self.app_state = AppState::Confirm,
            Popup::DeleteTodoItem => self.app_state = AppState::Confirm,
        };
    }

    pub fn confirm_popup(&mut self, value: bool) {
        if !value {
            self.close_popup();
            return;
        }

        match self.popup {
            Popup::DeleteTodo => {
                if !self.items.is_empty() {
                    self.items.remove(self.current_item_index);
                }
            }
            Popup::DeleteTodoItem => {
                if let Some(todo) = self.get_current_item_mut() {
                    if !todo.items.is_empty() {
                        todo.items.remove(todo.current_item_index);
                    }
                }
            }
            _ => {}
        }
        self.close_popup();
    }

    pub fn close_popup(&mut self) {
        self.input = Input::default();
        self.app_state = AppState::Normal;
        self.popup = Popup::None;
    }

    pub fn apply_popup(&mut self) {
        match self.popup {
            Popup::CreateTodo => {
                self.items.push(Todo::new(&self.input.value));
                self.close_popup();
            }
            Popup::RenameTodo => {
                let title = self.input.value.to_owned();
                if let Some(todo) = self.get_current_item_mut() {
                    todo.title = title;
                }
                self.close_popup();
            }
            Popup::CreateTodoItem => {
                self.create_todo_item();
                self.close_popup();
            }
            Popup::RenameTodoItem => {
                let text = self.input.value.to_owned();
                if let Some(todo) = self.get_current_item_mut() {
                    if let Some(todo_item) = todo.get_current_item_mut() {
                        todo_item.text = text;
                    }
                }
                self.close_popup();
            }
            _ => {}
        };
    }

    fn create_todo_item(&mut self) {
        let value = self.input.value.to_owned();

        if let Some(todo) = self.get_current_item_mut() {
            todo.items.push(TodoItem::new(&value));
        }
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
