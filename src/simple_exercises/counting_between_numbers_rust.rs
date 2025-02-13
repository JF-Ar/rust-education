use std::io;
use crate::utils::terminal::{clear, description};

fn get_numbers() -> (i32, i32) {

    clear();

    let descriptor = "You need to enter two numbers.\nAnd I will count down or up according to the first number.";

    println!("{}", description(descriptor));

    println!("Enter the a number: ");
    let mut number1_str = String::new();
    io::stdin().read_line(&mut number1_str).unwrap();
    let number1 = number1_str.trim().parse::<i32>().unwrap();

    println!("Enter other number: ");
    let mut number2_str = String::new();
    io::stdin().read_line(&mut number2_str).unwrap();
    let number2 :i32 = number2_str.trim().parse::<i32>().unwrap();

    return (number1, number2)
}

fn count_using_while(mut first_number: i32, second_number: i32) {

    if first_number > second_number {

        while first_number > second_number {

            first_number -= 1;

            println!("Counting {}",first_number)
        }

    } else {

        while first_number < second_number {

            first_number += 1;

            println!("Counting {}",first_number)
        }
    }
}

fn count_using_for(first_number: i32, second_number: i32) {

    let mut range_start = first_number;
    let mut range_end = second_number;

    if first_number > second_number {
        range_start = second_number;
        range_end = first_number + 1;
    }

    for num in range_start..range_end {
        println!("Counting {}", num);
    }
}

pub fn while_count() {

    let (first_number, second_number) = get_numbers();

    count_using_while(first_number, second_number);
}
pub fn for_count() {

    let (first_number, second_number) = get_numbers();

    count_using_for(first_number, second_number);
}