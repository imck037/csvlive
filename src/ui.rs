use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_ui(frame: &mut Frame) {
    let text = Paragraph::new("this is csv live, here you can edit any csv file.")
        .block(Block::default().title("Csv Live").borders(Borders::all()))
        .centered();
    frame.render_widget(text, frame.area());
}
