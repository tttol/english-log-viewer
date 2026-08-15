use std::io;

mod application;
mod presentation;

fn main() -> io::Result<()> {
    ratatui::run(application::app::run)
}
