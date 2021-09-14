mod entry;
mod entry_builder;
mod io;

mod prelude {
    pub use crate::entry::Entry;
    pub use crate::entry_builder::*;
    pub use crate::io::*;
}

use prelude::*;
use std::io::stdin;
use std::panic::resume_unwind;

const COMMAND_ADD: &str = "add";
const COMMAND_DONE: &str = "done";

fn main() {
    loop {
        println!("Add entry:");

        let mut raw_entry = String::new();
        stdin().read_line(&mut raw_entry).expect("Failed to read entry_builder");

        let entry_result = match raw_entry.split_once(" ") {
            Some((cmd, rest)) => match cmd.to_ascii_lowercase().as_str() {
                COMMAND_ADD => EntryBuilder::from_str(rest),
                _ => Err(format!("Unknown command: {}", cmd)),
            },
            _ => match raw_entry.trim() {
                COMMAND_DONE => break,
                _ => Err(format!("Invalid entry format: {}", raw_entry)),
            },
        };

        match entry_result {
            Ok(e) => println!("Entry created: {:?}", e),
            Err(err) => println!("Error creating entry: {}", err),
        }
    }
}


