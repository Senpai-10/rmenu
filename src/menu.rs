use crate::help::help;
use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const SELECTED_PAIR: i16 = 1;

pub struct Menu {
    list: Vec<String>,
    selected: Vec<usize>,
    /// Cursor position in list using index
    cursor: usize,
    quit: bool,
    settings: Settings,
}

pub struct Settings {
    pub multi_select: bool,
}

impl Menu {
    pub fn new(list: Vec<String>, settings: Settings) -> Menu {
        Menu {
            list: list,
            selected: Vec::new(),
            cursor: 0,
            quit: false,
            settings: settings,
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
            'a' => self.select_all(),
            'A' => self.unselect_all(),
            'h' | '?' => help(),
            'q' => self.exit(),
            _ => {}
        }
    }

    fn display_list(&self) {
        for (index, item) in self.list.iter().enumerate() {
            let pair = {
                if self.selected.contains(&index) {
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

    fn done(&mut self) {
        // Exiting ncurses window to be able to show print output
        self.exit();

        if self.selected.is_empty() {
            println!("{}", self.list[self.cursor]);
        } else {
            // Output is ordered by what has been selected first

            // Uncomment this code to sort output by index number
            // self.selected.sort();

            for (_, i) in self.selected.iter().enumerate() {
                println!("{}", self.list[*i]);
            }
        }
    }

    fn select(&mut self) {
        if !self.settings.multi_select {
            return ();
        }

        let element_index = self.cursor;

        if self.selected.contains(&element_index) {
            // Find index for element {self.cursor}
            let index = self
                .selected
                .iter()
                .position(|x| *x == self.cursor)
                .unwrap();
            mv(5, 0);
            addstr(&format!("remove index: {}", index));
            refresh();

            self.selected.remove(index);
        } else {
            self.selected.push(element_index);
        }
    }

    /// Add all self.list elements in self.selected
    fn select_all(&mut self) {
        if !self.settings.multi_select {
            return ();
        }

        for (i, _) in self.list.iter().enumerate() {
            self.selected.push(i);
        }
    }

    /// Empty self.selected
    fn unselect_all(&mut self) {
        if !self.settings.multi_select {
            return ();
        }

        self.selected.clear();
    }

    // Return the last index in self.selected Vector, without overflow
    fn last_index(&self) -> usize {
        if self.selected.len() == 0 {
            return 0;
        } else {
            return self.selected.len() - 1;
        }
    }

    fn exit(&mut self) {
        self.quit = true;
        endwin();
    }
}
