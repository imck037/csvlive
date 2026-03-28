use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Paragraph, Row, Table},
};

use crate::{App, Mode};

pub fn render_ui(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(2)])
        .split(area);

    let visible_height = layout[0].height as usize;

    let start = if app.selected_row >= visible_height {
        app.selected_row - visible_height + 1
    } else {
        0
    };

    let rows = app
        .contents
        .iter()
        .skip(start)
        .take(visible_height)
        .enumerate()
        .map(|(i, row)| {
            let cells = row.iter().enumerate().map(|(j, cell)| {
                if i + start == app.selected_row && j == app.selected_coloumn {
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

    let table = Table::new(rows, widths).block(Block::default());
    frame.render_widget(table, layout[0]);

    let status_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(1); 2])
        .split(layout[1]);

    let status_bar = match app.mode {
        Mode::Normal => String::from("Normal i: insert mode"),
        Mode::Insert => String::from("Insert ESC: cancel edit, <CR>: save edit"),
    };

    let status_widget = Paragraph::new(status_bar).block(
        Block::default().style(
            Style::default()
                .bg(Color::Blue)
                .black()
                .add_modifier(Modifier::BOLD),
        ),
    );

    frame.render_widget(status_widget, status_layout[0]);

    let input_bar = match app.mode {
        Mode::Normal => Paragraph::new("j: down, k: up, l: right, h: left"),
        Mode::Insert => Paragraph::new(format!("Edit: {}", app.input)),
    };

    frame.render_widget(input_bar, status_layout[1]);
}
