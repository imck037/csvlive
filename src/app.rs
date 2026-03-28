use crate::Mode;

pub(crate) struct App {
    pub contents: Vec<Vec<String>>,
    pub selected_row: usize,
    pub selected_coloumn: usize,
    pub mode: Mode,
    pub input: String,
}

impl App {
    pub fn build(contents: Vec<Vec<String>>) -> Self {
        Self {
            contents,
            selected_row: 1,
            selected_coloumn: 0,
            mode: Mode::Normal,
            input: String::new(),
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_row > 1 {
            self.selected_row -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_row < self.contents.len() - 1 {
            self.selected_row += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.selected_coloumn > 0 {
            self.selected_coloumn -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.selected_coloumn < self.contents[0].len() - 1 {
            self.selected_coloumn += 1;
        }
    }

    pub fn start_edit(&mut self) {
        self.mode = Mode::Insert;
        self.input = self.contents[self.selected_row][self.selected_coloumn].clone();
    }

    pub fn save_edit(&mut self) {
        self.contents[self.selected_row][self.selected_coloumn] = self.input.clone();
        self.mode = Mode::Normal;
    }

    pub fn cancel_edit(&mut self) {
        self.input.clear();
        self.mode = Mode::Normal;
    }
}
