/*

TODO setup ncurses first with Vector of test items in the code
TODO use stdin

*/

mod menu;

use menu::Menu;

fn main() {
    let list = vec![
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
    ];

    let mut menu = Menu::new(list);
    menu.start();
}
