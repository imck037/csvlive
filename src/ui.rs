use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

use crate::{App, Mode};

pub fn render_ui(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(2)])
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

    let widths: Vec<Constraint> = app.contents[0]
        .iter()
        .map(|_| Constraint::Percentage(100 / app.contents[0].len() as u16))
        .collect();

    let table =
        Table::new(rows, widths).block(Block::default().title("CSV Live").borders(Borders::all()));
    frame.render_widget(table, layout[0]);

    let status_bar = match app.mode {
        Mode::Normal => String::from("Normal i: insert mode"),
        Mode::Insert => String::from("Insert ESC: cancel edit, <CR>: save edit"),
    };

    let input_bar = match app.mode {
        Mode::Normal => String::from("j: down, k: up, l: right, h: left"),
        Mode::Insert => String::from(format!("Edit: {}", app.input)),
    };

    let status: Vec<String> = vec![status_bar, input_bar];

    frame.render_widget(Paragraph::new(status.join("\n")), layout[1]);
}
