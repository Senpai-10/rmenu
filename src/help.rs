use ncurses::*;

pub fn help() {
    clear();

    let mut y: i32 = 0;

    help_print(&mut y, "--- Help ---");
    help_print(&mut y, "");
    help_print(&mut y, "Move Up:        w, k");
    help_print(&mut y, "Move Down:      s, j");
    help_print(&mut y, "Select:         space_key");
    help_print(&mut y, "Select All:     a");
    help_print(&mut y, "Unselect All:   A (shift + a)");
    help_print(&mut y, "Done:           enter_key");
    help_print(&mut y, "Help:           h, ?");
    help_print(&mut y, "Quit:           q");

    getch();
}

fn help_print(y: &mut i32, msg: &str) {
    mv(*y, 1);
    addstr(msg);

    *y += 1;
}
