use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Widget, Wrap},
};

use crate::{
    models::state::{Popup, State},
    ui::todo_list,
};

pub fn draw_main(frame: &mut Frame, rect: Rect, state: &State) {
    let block = Block::new()
        .title_top(Line::raw("Todo App").alignment(Alignment::Center))
        .borders(Borders::all())
        .title_bottom(Line::from(vec![Span::raw("q - quit")]).alignment(Alignment::Center));

    let inner_area = block.inner(rect);

    frame.render_widget(block, rect);

    let rects = Layout::new(
        ratatui::layout::Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ],
    )
    .split(inner_area);

    let tabs: Vec<String> = state.items.iter().map(|i| i.title.to_owned()).collect();
    Tabs::new(tabs)
        .select(state.get_current_item_index())
        .render(rects[0], frame.buffer_mut());

    todo_list::draw(frame, rects[2], state);

    match state.popup {
        Popup::None => {}
        Popup::CreateTodo => {
            render_create_widget(frame, rect, state, "Create Todo");
        }
        Popup::RenameTodo => {
            render_create_widget(frame, rect, state, "Edit Todo");
        }
        Popup::CreateTodoItem => {
            render_create_widget(frame, rect, state, "Create Todo Item");
        }
        Popup::RenameTodoItem => {
            render_create_widget(frame, rect, state, "Edit Todo Item");
        }
        Popup::DeleteTodo => {
            render_confirm_widget(frame, rect, "Delete Todo");
        }
        Popup::DeleteTodoItem => {
            render_confirm_widget(frame, rect, "Delete Todo Item");
        }
    }
}

fn render_create_widget(frame: &mut Frame, rect: Rect, state: &State, message: &str) {
    let block = Block::bordered().title(message);
    let area = popup_area(rect, 60, 20);

    let (input_text, style) = if state.input.value.is_empty() {
        ("Enter name...", Style::default().fg(Color::Gray))
    } else {
        (
            state.input.value.as_ref(),
            Style::default().fg(Color::White),
        )
    };

    let inner = block.inner(area);
    let input = Paragraph::new(input_text).style(style).block(block);

    frame.set_cursor_position(Position::new(
        inner.x + state.input.get_cursor_position() as u16,
        inner.y,
    ));

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}

fn render_confirm_widget(frame: &mut Frame, rect: Rect, message: &str) {
    let block = Block::bordered().title("Confirm");
    let area = popup_area(rect, 60, 30);

    frame.render_widget(Clear, area);

    let inner = block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Message area
            Constraint::Length(1), // Spacing
            Constraint::Length(1), // Instructions
        ])
        .split(inner);

    let message_widget = Paragraph::new(message)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    let instructions = Paragraph::new("Press 'y' to confirm, 'n' to cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    frame.render_widget(block, area);
    frame.render_widget(message_widget, chunks[0]);
    frame.render_widget(instructions, chunks[2]);
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
