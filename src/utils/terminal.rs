use std::io::Write;

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}

pub fn show_menu(title_input: &str, items: &[&str], out: bool) -> u32 {

    let title = String::from("JF-Ar Rust Program :: ") + title_input;
    println!("{}", title);
    println!("{}", String::from("=").repeat(title.len()));

    show_items(items);

    println!("{}", if out { "* - Bye" } else { "* - Go Back" });
    print!("\nChoose an option: ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let choice: Result<u32, _> = line.trim().parse();

    match choice {
        Ok(choice) => choice,
        _ => 0,
    }
}

fn show_items(items: &[&str]) {
    for (i, item) in items.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}

pub fn description(description: &'static str) -> &'static str {

    return  description;
}