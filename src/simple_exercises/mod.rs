use std::process::exit;
use crate::utils::terminal::{show_menu};

mod counting_between_numbers_rust;
mod largest_number_found;
mod prime_number;

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

