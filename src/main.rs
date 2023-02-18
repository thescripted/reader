use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    let command = match args.get(1) {
        Some(command) => command,
        None => {
            print_help();
            return Ok(());
        }
    };
    if command == "help" {
        print_help();
    } else if command == "add" {
        let book = match args.get(2) {
            Some(book) => book,
            None => {
                println!("Please enter a book name");
                return Ok(());
            }
        };

        let file_name = "book.txt";
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(file_name)?;

        file.write_all(book.as_bytes())?;
        file.write_all(b"\n")?;
    } else if command == "remove" {
    } else if command == "list" {
    } else {
        println!("Invalid command");
    }
    Ok(())
}

fn print_help() {
    let help_message = "usage: reader <command> [options]

list of available commands: 
add <book>          Add a book to the list
remove <book>       Remove a book from the list
list                List all books in the list
help                Show this help message

examples:
reader add \"The Lord of the Rings\"
reader remove \"The Lord of the Rings\"
reader list
";
    println!("{}", help_message);
}
