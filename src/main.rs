use rusqlite::Connection;
use std::env;
use std::error::Error;

#[derive(Debug)]
struct Book {
    title: String,
}

const DATABASE_PATH: &'static str = "books.db";

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
        let conn = match Connection::open(DATABASE_PATH) {
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

        // TODO(ben): for SQL queries, singular book or plural books?
        match conn.execute("INSERT INTO books (title) VALUES (?)", [&book.to_string()]) {
            Ok(_) => println!("book added"),
            Err(e) => println!("Error: {}", e),
        };
    } else if command == "remove" {
        let conn = match Connection::open(DATABASE_PATH) {
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

        match conn.execute("DELETE FROM books WHERE title = ?", [&book.to_string()]) {
            Ok(_) => println!("book removed"),
            Err(e) => println!("Error: {}", e),
        };
    } else if command == "list" {
        let conn = match Connection::open(DATABASE_PATH) {
            Ok(conn) => conn,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(());
            }
        };

        let mut stmt = conn.prepare("SELECT title FROM books")?;
        let book_iter = stmt.query_map([], |row| Ok(Book { title: row.get(0)? }))?;
        book_iter.for_each(|book| {
            let title = match book {
                Ok(book) => book.title,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            };
            println!("{}", title);
        });
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
