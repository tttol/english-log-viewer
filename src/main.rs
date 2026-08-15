use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Paragraph, Row, Table},
};

fn main() -> io::Result<()> {
    ratatui::run(run)
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(render)?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && key.code == KeyCode::Char('q')
        {
            return Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let [header_area, table_area, footer_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .areas(frame.area());

    let header = Paragraph::new("English Log Viewer")
        .alignment(Alignment::Center)
        .block(Block::bordered());
    let table = Table::new(
        Vec::<Row>::new(),
        [Constraint::Length(25), Constraint::Min(20)],
    )
    .header(Row::new(["Reviewed at", "Feedback"]))
    .block(Block::bordered().title(" Reviews "));
    let footer = Paragraph::new("q Quit")
        .alignment(Alignment::Center)
        .block(Block::bordered());

    frame.render_widget(header, header_area);
    frame.render_widget(table, table_area);
    frame.render_widget(footer, footer_area);
}
