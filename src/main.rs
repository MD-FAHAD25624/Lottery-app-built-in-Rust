use rand::RngExt; //Written for rand v0.10.2
use std::fs;
use std::io;
use std::path::Path;
const SAVE_NAMES: &str = "names.txt";

fn save_names(list: &[String]) {
    let content = list.join("\n");

    match fs::write(SAVE_NAMES, content) {
        Ok(_) => {}
        Err(e) => println!("Warning: Could not save names: {}", e),
    }
}
fn load_names() -> Vec<String> {
    // If the file doesn't exist yet, return an empty Vec
    if !Path::new(SAVE_NAMES).exists() {
        return Vec::new();
    }
    let content = match fs::read_to_string(SAVE_NAMES) {
        Ok(c) => c,
        Err(e) => {
            println!("Warning: Could not load names: {}", e);
            return Vec::new();
        }
    };

    let list = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().to_string())
        .collect();
    list
}

fn show_names(list: &[String]) {
    if list.is_empty() {
        println!("No names to show! Add some names first.");
        return;
    }
    println!("All names:");
    for (i, name) in list.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }
}

fn capitalise_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

fn add_names(list: &mut Vec<String>) {
    println!("Existing names:");
    show_names(list);
    loop {
        loop {
            println!("Enter the name to add or 0 to exit:");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Failed.");
            if name.trim() == "0" {
                break;
            }
            if name.trim().is_empty() {
                continue;
            }
            let name_f = capitalise_first(name.trim());
            if list.contains(&name_f) {
                println!("Name already available.");
                continue;
            }
            list.push(name_f);
        }
        println!("New name list:");
        show_names(list);
        save_names(list);
        println!("Do you want to go back to the main menu?Write 'Yes' to go back.");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed");
        match choice.trim().to_lowercase().as_str() {
            "yes" | "y" => {
                println!("Going back to the main menu:");
                break;
            }
            _ => continue,
        }
    }
}

fn lottery(list: &[String]) {
    println!("Do lotteries here!(Repeat possible, wheel style)");
    show_names(list);
    if list.is_empty() {
        println!("Returning to the main menu...");
        return;
    }
    loop {
        println!("How many names to pick? Enter the number or q to exit:");
        let mut count_input = String::new();
        io::stdin().read_line(&mut count_input).expect("Failed");
        if count_input.trim() == "q" || count_input.trim() == "Q" {
            println!("Returning...");
            break;
        }
        let count: usize = match count_input.trim().parse() {
            Ok(n) if n >= 1 && n <= list.len() => n,
            Ok(_) | Err(_) => {
                println!(
                    "Error: Please enter a valid number between 1 and {}.",
                    list.len()
                );
                continue;
            }
        };

        let mut picked: Vec<&String> = Vec::new();
        let mut rng = rand::rng();
        for _ in 0..count {
            picked.push(&list[rng.random_range(0..list.len())]);
        }
        for (i, name) in picked.iter().enumerate() {
            println!("{}. {}", i + 1, name);
        }
        println!("Do you want to go back to the main menu? Type 'yes' to go back:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed");
        match choice.trim().to_lowercase().as_str() {
            "yes" | "y" => {
                println!("Going back to the main menu.");
                break;
            }
            _ => continue,
        }
    }
}
fn del_names(list: &mut Vec<String>) {
    if list.is_empty() {
        println!("No names to delete.");
        return;
    }
    show_names(list);
    let mut indices_to_delete: Vec<usize> = Vec::new();
    loop {
        println!("Enter the name number to delete or q to exit:");
        let mut del_in = String::new();
        io::stdin().read_line(&mut del_in).expect("Failed");
        if del_in.trim() == "q" || del_in.trim() == "Q" {
            println!("Returning...");
            break;
        }
        let delete: usize = match del_in.trim().parse() {
            Ok(n) if n >= 1 && n <= list.len() => n,
            Ok(_) | Err(_) => {
                println!(
                    "Invalid input,enter a valid number between 1 and {}",
                    list.len()
                );
                continue;
            }
        };
        let idx = delete - 1;
        if indices_to_delete.contains(&idx) {
            println!("Name number {} is already marked for deletion.", idx + 1);
            continue;
        }
        indices_to_delete.push(idx);
        let pending: Vec<String> = indices_to_delete
            .iter()
            .map(|i| (i + 1).to_string())
            .collect();
        println!("Marked for deletion: {}", pending.join(", "));
    }
    //Remove in reverse order so that the indices stay valid
    indices_to_delete.sort_unstable_by(|a, b| b.cmp(a));
    println!("Deleting...");
    for i in indices_to_delete {
        list.remove(i);
    }
    println!("Updated list:");
    show_names(list);
    save_names(list);
}
fn main() {
    let mut list: Vec<String> = load_names();
    loop {
        println!("{}", "=".repeat(42));
        println!("          LOTTERY APP ");
        println!("{}", "=".repeat(42));
        println!("Here you can do lotteries!\n");
        println!("Choose an option:");
        println!("1. Add name");
        println!("2. See all names");
        println!("3. Do lotteries");
        println!("4. Delete names");
        println!("5. Exit");
        let mut choice_input = String::new();
        io::stdin().read_line(&mut choice_input).expect("Failed");
        let choice: usize = match choice_input.trim().parse() {
            Ok(n) if n >= 1 && n <= 5 => n,
            Ok(_) | Err(_) => {
                println!("Error: Please enter a number between 1 and 5.");
                continue;
            }
        };
        match choice {
            1 => add_names(&mut list),
            2 => show_names(&list),
            3 => lottery(&list),
            4 => del_names(&mut list),
            5 => {
                println!("Thank you for using the lottery app, goodbye!");
                break;
            }
            _ => {}
        }
    }
}
