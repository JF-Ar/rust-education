use std::process::exit;
use crate::utils::terminal::show_menu;

mod counting_between_numbers_rust;

pub fn counting_numbers() {

    let items = ["Counting Use While", "Counting Use For"];

    let menu = show_menu("Counting", &items, true);

    println!("You need to enter two numbers.\n And I will count down or up according to the first number.");

    match menu {
        1 => counting_between_numbers_rust::while_count(),
        2 => counting_between_numbers_rust::for_count(),
        _ => exit(0),
    }
}

