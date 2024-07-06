use std::io;
use crate::utils::terminal::{clear, description};

fn get_input() -> String {

    clear();

    let descriptor = "You need to enter any word.\nAnd I will tell you if this word has unique characters or not";

    println!("{}", description(descriptor));

    println!("Enter a word: ");
    let mut word = String::new();
    io::stdin().read_line(&mut word).unwrap();
    let word_check: String = word.trim().to_string();
    return word_check;
}

fn has_unique_characters(word: &String) -> &'static str {

    let mut char_list: Vec<char> = Vec::new();

    for c in word.chars() {

        if char_list.contains(&c) {

            return "Does not have unique characters";
        }

        char_list.push(c);
    }

    return "Has unique characters";
}

fn has_unique_characters_128(word: &String) -> (String, bool){

    let mut char_list = [false; 128];

    for &c in word.as_bytes() {

        let index = c as usize;

        if char_list[index] {

            let string_formated = format!("Has duplicated characters {}", c as char);

            return  (string_formated , false);
        }

        char_list[index] = true;
    }

    return ("Has unique characters".to_string(), true);
}

pub fn return_unique_characters() {

    let word = get_input().clone();
    let result = has_unique_characters(&word);

    println!("\nThe word {} \n {}\n", word, result);
    println!("------------")
}

pub fn return_unique_characters_128() {

    let word = get_input().clone();
    let result = has_unique_characters_128(&word);

    println!("\nThe word {} \n {:?}\n", word, result);
    println!("------------")
}