mod utils;
mod simple_exercises;

use std::process::exit;
use utils::terminal::{clear, show_menu};

fn main() {

    clear();

    loop {

        let items = ["Counting Numbers", "Find Highest Number in a Vector"];
        let menu = show_menu("Principal", &items, true);

        clear();

        match menu {
            1 => simple_exercises::counting_numbers(),
            2 => simple_exercises::largest_number_in_the_vector(),
            3 => exit(0),
            4 => exit(0),
            5 => exit(0),
            _ => exit(0),
        }
    }
}