use mylibrary::MyLibrary;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Credits {
    code: [char; 4usize],
    name: String,
    date: NaiveDate
}

impl MyLibrary<Credits> for Credits {
    fn create_struct(path_file: &str) -> Vec<Credits> {
        let mut credits: Vec<Credits>= vec![];
        for item in Credits::read_file(path_file) {
            let split_row: Vec<&str> = item.split(" ").collect();

            if split_row.len() != 4 {
                let book = Credits {
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

impl Credits {

    pub fn get_books(path_file: &str) -> Vec<Credits> {
        Credits::create_struct(path_file)
    }

    pub fn print_books(books: &Vec<Credits>){
        for book in books {
            println!("{:?}", book);
        }
    }
}