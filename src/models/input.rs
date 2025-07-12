#[derive(Default)]
pub struct Input {
    pub value: String,
    cursor_position: usize,
}

impl Input {
    pub fn handle_input(&mut self, c: char) {
        if self.cursor_position > self.value.len() {
            self.cursor_position = self.value.len();
        }
        self.value.insert(self.cursor_position, c);
        self.right();
    }

    pub fn left(&mut self) {
        if self.cursor_position == 0 {
            return;
        }
        self.cursor_position -= 1;
    }

    pub fn right(&mut self) {
        self.cursor_position += 1;
        if self.cursor_position >= self.value.len() {
            self.cursor_position = self.value.len();
        }
    }

    pub fn backspace(&mut self) {
        if self.value.is_empty() {
            return;
        }

        self.value.remove(self.cursor_position - 1);
        self.left();
    }

    pub fn delete(&mut self) {
        if self.value.is_empty() {
            return;
        }

        self.value.remove(self.cursor_position);
        self.right();
    }
}
