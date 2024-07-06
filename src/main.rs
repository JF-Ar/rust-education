mod utils;
mod simple_exercises;

use std::process::exit;
use utils::terminal::{clear, show_menu};

fn main() {

    clear();

    loop {

        let items = [
            "Counting Numbers",
            "Find Highest Number in a Vector",
            "Number is Prime",
            "Multiplication Table",
            "Unique Characters",
        ];
        let menu = show_menu("Principal", &items, true);

        clear();

        match menu {
            1 => simple_exercises::counting_numbers(),
            2 => simple_exercises::largest_number_in_the_vector(),
            3 => simple_exercises::check_prime_number(),
            4 => simple_exercises::multiplication_table(),
            5 => simple_exercises::has_unique_characters(),
            _ => exit(0),
        }
    }
}