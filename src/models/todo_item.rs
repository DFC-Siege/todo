use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    done: bool,
    pub text: String,
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            done: false,
            text: text.to_owned(),
        }
    }

    pub fn toggle(&mut self) {
        self.done = !self.done;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}
