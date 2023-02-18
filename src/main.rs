use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter a command"); // maybe output the help message here
        return Ok(());
    }

    let command = &args[1];
    if command == "help" {
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
    } else if command == "add" {
        let _book = match args.get(2) {
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

        file.write_all(b"Hello, world!")?;
        println!("Add message");
    } else if command == "remove" {
        println!("Remove message");
    } else if command == "list" {
        println!("List message");
    } else {
        println!("Invalid command");
    }
    Ok(())
}
