use std::io;
use crate::utils::terminal::{clear, description};

fn get_vector() -> Vec<i32> {

    clear();

    let descriptor = "You need to enter a integer vector like: 1,2,3.\nAnd I will show to you the largest value in this vector";

    println!("{}", description(descriptor));

    println!("Enter the vector: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Falha ao ler o input");
    let input = input.trim();

    let numbers: Vec<i32> = input
        .split(',')
        .map(|s| s.parse().expect("Sorry! We can't convert this"))
        .collect();

    return numbers;
}

fn get_largest_value(vector: Vec<i32>) {

    let num = vector.iter().max();

    println!("In this vector: {:?} the largest value is {}\n", vector, num.unwrap().abs());
    println!("----------")
}

pub fn return_larges_value() {

    let vector = get_vector();
    get_largest_value(vector);
}
