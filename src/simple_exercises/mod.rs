use std::process::exit;
use crate::utils::terminal::{show_menu};

mod counting_between_numbers_rust;
mod largest_number_found;

pub fn counting_numbers() {

    let items = ["Counting Use While", "Counting Use For"];

    let menu = show_menu("Counting", &items, true);

    match menu {
        1 => counting_between_numbers_rust::while_count(),
        2 => counting_between_numbers_rust::for_count(),
        _ => exit(0),
    }
}

pub fn largest_number_in_the_vector() {

    let items = ["Return Largest Value of Vector"];

    let menu = show_menu("Counting", &items, true);

    match menu {
        1 => largest_number_found::return_larges_value(),
        _ => exit(0),
    }
}

