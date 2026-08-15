use std::io;

use ratatui::{DefaultTerminal, crossterm::event::{self, Event, KeyCode, KeyEventKind}};

use crate::presentation;


pub fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(presentation::render::render)?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && key.code == KeyCode::Char('q')
        {
            return Ok(());
        }
    }
}
