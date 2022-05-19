mod help;
mod menu;

use clap::Parser;
use menu::{Menu, Settings};

fn main() {
    let args = Args::parse();
    let mut list: Vec<String> = Vec::new();

    if args.confirm && args.list.is_empty() {
        list.push(String::from("yes"));
        list.push(String::from("no"));
    } else {
        list = args.list;
    }

    let mut menu = Menu::new(
        list,
        Settings {
            multi_select: args.multi_select,
        },
    );
    menu.start();
}

/// Simple terminal select menu generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// List of menu items
    #[clap(short = 'l', long, multiple_occurrences = true, min_values = 2)]
    list: Vec<String>,

    /// Prompt for yes or no.
    /// this flag is won't work if a list is provided
    #[clap(short = 'c', long)]
    confirm: bool,

    /// Enable multi selection
    #[clap(short = 'm', long)]
    multi_select: bool,
}
