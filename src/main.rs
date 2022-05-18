/*

TODO setup ncurses first with Vector of test items in the code
TODO use stdin

*/

mod menu;

use menu::Menu;

fn main() {
    let list = vec![String::from("yes"), String::from("no")];

    let mut menu = Menu::new(list);
    menu.start();
}
