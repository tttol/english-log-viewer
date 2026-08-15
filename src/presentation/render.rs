use ratatui::{layout::{Alignment, Constraint, Layout}, widgets::{Block, Paragraph, Row, Table}};

pub fn render(frame: &mut Frame) {
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
