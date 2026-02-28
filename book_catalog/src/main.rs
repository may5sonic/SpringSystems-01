use std::fs::File;
use std::io::{BufRead, BufReader, Write};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    //Create the file, or truncate (overwrite) it if it already exists
    let mut file = File::create(filename).expect("Failed to create file");

    // Write each book to the file, seperated by commas
    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year).expect("Failed to write to file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let mut loaded_books = Vec::new();

    // Open the file for reading
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        // Split the line by commas to get the individual fields
        let parts: Vec<&str> = line.split(',').collect();

        // Ensure we have exactly 3 parts before trying to create a Book
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            
            // Parse the year string back into a u16 integer
            let year: u16 = parts[2].parse().expect("Failed to parse year");

            loaded_books.push(Book {
                title,
                author,
                year,
            });
        }
    }
    loaded_books
}

fn main() {

    // intialize a vector with some sample Book instance to test our functions
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year:1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year:1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books: ");

    for book in loaded_books {
        println!("'{}' by {}, published in {}", book.title, book.author, book.year);
    }
}
