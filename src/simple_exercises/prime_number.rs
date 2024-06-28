use std::io;
use crate::utils::terminal::{clear, description};

pub fn get_prime_number() {

    clear();

    let descriptor = "You need to enter a number.\nAnd I will tell if this number is prime or not";

    println!("{}", description(descriptor));

    println!("Enter the a number: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();
    let format_number = number.trim().parse::<i32>().unwrap();

    let result = is_prime_number(format_number);

    println!("\nThe number {}{}\n", number, result);
    println!("------------")
}

fn is_prime_number(num: i32) -> &'static str {

    if num <= 1 {

        return "Is not prime";
    }

    let limit = (num as f32).sqrt() as i32 + 1;

    for i in 2..limit{

        if num % i == 0 {

            return "Is Not prime";
        }
    }

    &"Is prime"
}