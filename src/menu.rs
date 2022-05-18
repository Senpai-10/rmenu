use ncurses::*;

pub struct Menu {
    list: Vec<String>,
    choices: Vec<String>,

    /// Cursor position in list using index
    cursor: usize,
    quit: bool,
}

impl Menu {
    pub fn new(list: Vec<String>) -> Menu {
        Menu {
            list: list,
            choices: Vec::new(),
            cursor: 0,
            quit: false,
        }
    }

    pub fn start(&mut self) {
        initscr();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        while !self.quit {
            clear();
            refresh();
            self.display_list();
            self.handle_input();
        }

        endwin();
    }

    fn list_up(&mut self) {
        if self.cursor != 0 {
            self.cursor -= 1;
        }
    }

    fn list_down(&mut self) {
        if self.cursor != self.list.len() - 1 {
            self.cursor += 1;
        }
    }

    fn handle_input(&mut self) {
        match getch() as u8 as char {
            'w' | 'k' => self.list_up(),
            's' | 'j' => self.list_down(),
            '\n' => self.done(),
            ' ' => self.select(),
            'q' => self.quit = true,
            _ => {}
        }
    }

    fn display_list(&self) {
        for (index, item) in self.list.iter().enumerate() {
            if self.cursor == index {
                mv(index as i32, 0);
                addstr(&format!("> {}", item));
            } else {
                mv(index as i32, 0);
                addstr(&format!("  {}", item));
            }
        }
    }

    fn done(&self) {
        // TODO if self.choices is empty output self.list[self.cursor]
        // TODO if self.choices is not empty iter the list and output self.list[choices[i]]
    }

    fn select(&self) {
        // TODO save current cursor position (index)
    }
}
