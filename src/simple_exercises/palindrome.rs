use std::io;
use crate::utils::terminal::{clear, description};

fn get_number() -> i32 {

    clear();

    let descriptor = "You need to enter a number.\nAnd I will check if it is a palindrome.";

    println!("{}", description(descriptor));

    println!("Enter a number: ");
    let mut number1_str = String::new();
    io::stdin().read_line(&mut number1_str).unwrap();
    let number1 = number1_str.trim().parse::<i32>().unwrap();

    return number1;
}

fn is_palindrome(number: i32) -> bool {

    let number_str = number.to_string();

    let reversed_number = number_str.chars().rev().collect::<String>();

    return number_str == reversed_number;
}

pub fn check_palindrome() {

    let number = get_number();

    let is_palindrome = is_palindrome(number);

    if is_palindrome {
        println!("The number {} is a palindrome", number);
    } else {
        println!("The number {} is not a palindrome", number);
    }
}