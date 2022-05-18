use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const SELECTED_PAIR: i16 = 1;

pub struct Menu {
    list: Vec<String>,
    selected: Vec<usize>,
    /// Cursor position in list using index
    cursor: usize,
    quit: bool,
}

impl Menu {
    pub fn new(list: Vec<String>) -> Menu {
        Menu {
            list: list,
            selected: Vec::new(),
            cursor: 0,
            quit: false,
        }
    }

    pub fn start(&mut self) {
        initscr();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        start_color();
        init_pair(REGULAR_PAIR, COLOR_CYAN, COLOR_BLACK);
        init_pair(SELECTED_PAIR, COLOR_BLACK, COLOR_WHITE);

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
            // ' ' => self.select(),
            'q' => self.quit = true,
            _ => {}
        }
    }

    fn display_list(&self) {
        for (index, item) in self.list.iter().enumerate() {
            let pair = {
                if self.cursor == index {
                    SELECTED_PAIR
                } else {
                    REGULAR_PAIR
                }
            };

            if self.cursor == index {
                mv(index as i32, 0);
                attron(COLOR_PAIR(pair));
                addstr(&format!("> {}", item));
                attr_off(COLOR_PAIR(pair));
            } else {
                mv(index as i32, 0);
                attron(COLOR_PAIR(pair));
                addstr(&format!("  {}", item));
                attr_off(COLOR_PAIR(pair));
            }
        }
    }

    fn done(&self) {
        // TODO if self.selected is empty output self.list[self.cursor]
        // TODO if self.selected is not empty iter the list and output self.list[selected[i]]
    }

    fn select(&mut self) {
        todo!();
        // TODO save current cursor position (index)
        // TODO if already selected remove from self.selected
        self.selected.push(self.cursor);
    }
}
