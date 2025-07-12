use color_eyre::eyre::Result;

use crate::{app::App, state::State, todo::Todo, todo_item::TodoItem};

mod app;
mod state;
mod todo;
mod todo_item;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut state = State::default();
    let mut todo = Todo::new("todo");
    let todo_item = TodoItem::new("todo applicatie maken");
    todo.add_item(todo_item);
    state.add_item(todo);

    let mut terminal = ratatui::init();

    let result = App::new(state).run(&mut terminal);
    ratatui::restore();

    result
}
