use std::io::{self, Write};

#[derive(Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    is_issued: bool,
}

impl Book {
    fn new(title: &str, author: &str, isbn: &str) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            isbn: isbn.to_string(),
            is_issued: false,
        }
    }
    fn issue_book(&mut self) -> Option<Book> {
        if self.is_issued {
            None
        } else {
            self.is_issued = true;
            Some(self.clone())
        }
    }
    fn return_book(&mut self) {
        self.is_issued = false;
    }
    fn print_details(&self) {
        println!(
            "Title: {}, Author: {}, ISBN: {}, Issued: {}",
            self.title, self.author, self.isbn, self.is_issued
        );
    }
}

fn main() {
    fn get_input(bookinfo: &str) -> String {
        print!("{}", bookinfo);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
    let title = get_input("Enter the book title: ");
    let author = get_input("Enter the author name: ");
    let isbn = get_input("Enter the ISBN number: ");
    let mut book = Book::new(&title, &author, &isbn);
    println!("\nBefore issuing:");
    book.print_details();
    if let Some(issued_book) = book.issue_book() {
        println!("\nIssued Book:");
        issued_book.print_details();
    } else {
        println!("Book is already issued.");
    }
    book.return_book();
    println!("\nAfter returning:");
    book.print_details();
}