use std::fmt::{Display, Formatter, Result};

use ratatui::{
    style::{Color, Modifier, Style},
    widgets::ListItem,
};

#[derive(Debug, Clone)]
pub struct TodoItem {
    done: bool,
    text: String,
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

    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let checkbox = if self.done { "☑" } else { "☐" };
        write!(f, "{} {}", checkbox, self.text)
    }
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(item: &TodoItem) -> Self {
        let content = item.to_string();
        let style = if item.done {
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::CROSSED_OUT)
        } else {
            Style::default()
        };

        ListItem::new(content).style(style)
    }
}
