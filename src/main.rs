use rusqlite::Connection;
use std::env;
use std::error::Error;

#[derive(Debug)]
struct Book {
    title: String,
}

// not sure what Box<dyn Error> is, but I will find out later
fn main() -> Result<(), Box<dyn Error>> {
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
        let conn = match Connection::open_in_memory() {
            Ok(conn) => conn,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(());
            }
        };
        let book = match args.get(2) {
            Some(book) => book,
            None => {
                println!("Please enter a book name");
                return Ok(());
            }
        };

        match conn.execute(
            "INSERT INTO book (title) VALUES (?1)",
            [&book.to_string()],
        ) {
            Ok(_) => println!("book added"),
            Err(e) => println!("Error: {}", e),
        };

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
