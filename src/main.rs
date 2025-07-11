use std::io;

use crate::app::App;

mod app;
mod todo;
mod todo_item;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();

    result
}
