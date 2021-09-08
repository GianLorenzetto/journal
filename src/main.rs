use std::io::stdin;

mod entry;
use crate::entry::entry::Entry;

fn main() {
    println!("Add entries one line at a time");

    let mut entry = String::new();
    stdin().read_line(&mut entry).expect("Failed to read entry");

    let mut items: Vec<&str> = vec![];
    entry
        .split_whitespace()
        .for_each(|itm| items.push(itm.trim()));

    if items.len() < 4 {
        println!("Not enough parameters");
        return;
    }

    match items[0].to_lowercase().as_str() {
        "add" => add_new_entry(&items),
        _ => println!("Unknown command"),
    };


}

fn add_new_entry(items: &Vec<&str>) {
    let (_, summary_items) = items.split_at(3);
    let result = Entry::from(items[1], items[2], &summary_items.join(" "));
    //let entry =
    match result {
        Ok(e) => println!("{:?}", e),
        Err(err) => println!("Error: {}", err),
    };
    println!("Valid entry!");
}


