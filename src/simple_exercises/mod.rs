use std::process::exit;
use crate::utils::terminal::{show_menu};

mod counting_between_numbers_rust;
mod largest_number_found;
mod prime_number;
mod multiplication_table;
mod unique_characters;
mod palindrome;
mod rotate_vector;

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

    let menu = show_menu("Largest Value of Vector", &items, true);

    match menu {
        1 => largest_number_found::return_larges_value(),
        _ => exit(0),
    }
}
pub fn check_prime_number() {

    let items = ["Verify if a Number is Prime or Not"];

    let menu = show_menu("Prime or Not", &items, true);

    match menu {
        1 => prime_number::get_prime_number(),
        _ => exit(0),
    }
}

pub fn multiplication_table() {

    multiplication_table::get_multiplicative_table();
}

pub fn palindrome_number() {

    palindrome::check_palindrome();
}

pub fn has_unique_characters() {

    let items = ["Return Unique (Without Structure)", "Return Unique (With Structure)"];

    let menu = show_menu("Return Unique Characters", &items, true);

    match menu {
        1 => unique_characters::return_unique_characters(),
        2 => unique_characters::return_unique_characters_128(),
        _ => exit(0),
    }
}

