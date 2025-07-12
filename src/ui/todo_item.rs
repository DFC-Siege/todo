use crate::models::todo_item::TodoItem;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

pub fn create_list_item(todo_item: &TodoItem) -> ListItem {
    let (checkbox, text_style) = if todo_item.is_done() {
        (
            Span::raw("✓ "),
            Style::new().add_modifier(Modifier::CROSSED_OUT),
        )
    } else {
        (Span::raw("□ "), Style::new())
    };

    let content = Line::from(vec![checkbox, Span::styled(&todo_item.text, text_style)]);

    ListItem::new(content)
}
