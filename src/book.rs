mod mylibrary;
use crate::book::mylibrary::MyLibrary;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Book {
    code: [char; 4usize],
    title: String,
    author: String,
    category: [char; 1usize],
    year: i32,
    duplicate: u32
}

impl MyLibrary<Book> for Book {
    fn create_struct(path_file: &str) -> Vec<Book> {
        let mut books: Vec<Book>= vec![];
        for item in Book::read_file(path_file) {
            let split_row: Vec<&str> = item.split(" ").collect();

            if split_row.len() != 5 {
                let book = Book {
                    code: split_row[0].chars().collect::<Vec<_>>().try_into().expect("Error"),
                    title: split_row[1].to_string(),
                    author: split_row[2].to_string(),
                    category: split_row[3].chars().collect::<Vec<_>>().try_into().expect("Error"),
                    year: split_row[4].parse().expect("Error"),
                    duplicate: split_row[5].parse().expect("Error"),
                };

                books.push(book);
            } else {
                panic!("{} syntax not correct", item);
            }
        };
        books
    }
}

impl Book {

    pub fn get_books(path_file: &str) -> Vec<Book> {
        Book::create_struct(path_file)
    }

    pub fn print_books(books: &Vec<Book>){
        for book in books {
            println!("{:?}", book);
        }
    }
}