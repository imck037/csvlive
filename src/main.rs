use std::{env, fs, io, process};
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

struct App {
    contents: Vec<Vec<String>>,
    selected_row: usize,
    selected_coloumn: usize,
    mode: Mode,
    input: String,
}

impl App {
    fn build(contents: Vec<Vec<String>>) -> Self {
        Self {
            contents,
            selected_row: 1,
            selected_coloumn: 0,
            mode: Mode::Normal,
            input: String::new(),
        }
    }

    fn move_up(&mut self) {
        if self.selected_row > 1 {
            self.selected_row -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected_row < self.contents.len() - 1 {
            self.selected_row += 1;
        }
    }

    fn move_left(&mut self) {
        if self.selected_coloumn > 0 {
            self.selected_coloumn -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.selected_coloumn < self.contents[0].len() - 1 {
            self.selected_coloumn += 1;
        }
    }

    fn start_edit(&mut self) {
        self.mode = Mode::Insert;
        self.input = self.contents[self.selected_row][self.selected_coloumn].clone();
    }

    fn save_edit(&mut self) {
        self.contents[self.selected_row][self.selected_coloumn] = self.input.clone();
        self.mode = Mode::Normal;
    }

    fn cancel_edit(&mut self) {
        self.input.clear();
        self.mode = Mode::Normal;
    }
}

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
