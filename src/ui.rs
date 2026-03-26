use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::App;

pub fn render_ui(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(area);

    let rows = app.contents.iter().enumerate().map(|(i, row)| {
        let cells = row.iter().enumerate().map(|(j, cell)| {
            if i == app.selected_row && j == app.selected_coloumn {
                Cell::from(cell.clone()).style(Style::default().bg(Color::DarkGray))
            } else {
                Cell::from(cell.clone())
            }
        });
        Row::new(cells)
    });

    let table = Table::new(rows, [Constraint::Percentage(10); 10])
        .block(Block::default().title("CSV Live").borders(Borders::all()));
    frame.render_widget(table, layout[0]);
}
