use std::io;
use crate::utils::terminal::{clear, description};


pub fn get_multiplicative_table() {

    clear();

    let descriptor = "You need to enter a integer number.\nAnd I will display its multiplication table";

    println!("{}", description(descriptor));

    println!("Enter the a number: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    let format_number = number.trim().parse::<i32>().unwrap();

    multiplication_table(format_number);
}

fn multiplication_table(num: i32) -> &'static str {

    for i in 1..11 {

        println!("{} x {} = {}", num, i, num * i);
    }

    println!("Multiplication table displayed\n");

    &"Multiplication table displayed"
}