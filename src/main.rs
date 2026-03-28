use std::{env, fs, io, process};
mod app;
mod events;
mod ui;

use crossterm::{
    self,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{self, Terminal, prelude::CrosstermBackend};

enum Mode {
    Normal,
    Insert,
}

type App = app::App;

fn main() -> Result<(), io::Error> {
    let mut args = check_args(env::args());
    check_csv_file(&args[1]);
    let contents = fetch_data(&mut args[1]);

    let mut app = App::build(contents);
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|frame| ui::render_ui(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                save_file(&args[1], app.contents);
                break;
            }
            events::handle_events(key, &mut app);
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn check_args(args: env::Args) -> Vec<String> {
    if args.len() < 2 {
        eprintln!("Error: CSV file is not given.");
        process::exit(1);
    } else if args.len() > 2 {
        eprintln!("Error: Only need one argument but more is given.");
        process::exit(1);
    }
    args.into_iter().collect()
}

fn check_csv_file(filename: &String) {
    if !filename.trim().ends_with(".csv") {
        eprintln!("FileNameError: Need a csv file.");
        process::exit(1);
    }
}

fn fetch_data(filename: &mut String) -> Vec<Vec<String>> {
    match fs::read_to_string(filename) {
        Ok(content) => {
            let mut rows: Vec<Vec<String>> = Vec::new();
            for line in content.lines() {
                let fields: Vec<String> = line.split(",").map(|str| str.to_string()).collect();
                rows.push(fields);
            }
            rows
        }
        Err(e) => {
            eprintln!("Error opening the file {e}");
            process::exit(1);
        }
    }
}

fn save_file(filename: &String, contents: Vec<Vec<String>>) {
    let mut data = Vec::new();
    for line in contents {
        let field = String::from(line.join(","));
        data.push(field);
    }
    let content = String::from(data.join("\n"));
    fs::write(filename, content).unwrap();
}
