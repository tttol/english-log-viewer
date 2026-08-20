use ratatui::{Frame, layout::{Alignment, Constraint, Layout}, widgets::{Block, Borders, Paragraph, Row, Table}};

// TITLE
// BEFORE, AFTER, COMMENT, REVIEWED_AT
pub fn render(frame: &mut Frame) {
    let [title_area, review_area] = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50)
    ]).areas(frame.area());

    let [before_area, after_area, comment_area, reviewed_at_area] = Layout::vertical([
        Constraint::Min(1),
        Constraint::Min(1),
        Constraint::Min(1),
        Constraint::Length(1)
    ])
    .areas(review_area);

    let title = Block::default()
        .title("TITLE")
        .borders(Borders::ALL);

    frame.render_widget(title, title_area);


    ////////////////////////////////////////
    // let [header_area, table_area, footer_area] = Layout::vertical([
    //     Constraint::Length(3),
    //     Constraint::Min(1),
    //     Constraint::Length(3),
    // ])
    // .areas(frame.area());
    //
    // let header = Paragraph::new("English Log Viewer")
    //     .alignment(Alignment::Center)
    //     .block(Block::bordered());
    // let table = Table::new(
    //     Vec::<Row>::new(),
    //     [Constraint::Length(25), Constraint::Min(20)],
    // )
    // .header(Row::new(["Reviewed at", "Feedback"]))
    // .block(Block::bordered().title(" Reviews "));
    // let footer = Paragraph::new("q Quit")
    //     .alignment(Alignment::Center)
    //     .block(Block::bordered());
    //
    // frame.render_widget(header, header_area);
    // frame.render_widget(table, table_area);
    // frame.render_widget(footer, footer_area);
}
