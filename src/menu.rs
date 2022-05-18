use ncurses::*;

pub struct Menu {
    list: Vec<String>,
    choices: Vec<String>,

    /// Cursor position in list using index
    cursor: i32,
}

impl Menu {
    pub fn new(list: Vec<String>) -> Menu {
        Menu {
            list: list,
            choices: Vec::new(),
            cursor: 0,
        }
    }

    pub fn start(&self) {
        initscr();

        mv(0, 0);
        addstr("hi");

        refresh();
        getch();
        endwin();
    }

    fn list_up(&self) {}
    fn list_down(&self) {}
}
