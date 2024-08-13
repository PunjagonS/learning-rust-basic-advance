// --------------------------------------------
//          IntoIterator
// --------------------------------------------

/*
    If Iterator is implemented on a type which you can interate over so 
    the IntoIterator is implemented on type which can be turn into Iterator.
*/

// trait IntoIterator {
//     type Item;
//     type IntoIterator: Iterator;
//     fn into_iter(self) -> Self::IntoIterator;
// }

use std::vec::IntoIter;

struct Book {
    title: String,
    author: String,
    genre: String,
}

// struct BookIterator {
//     properties: Vec<String>,
// }

// impl Iterator for BookIterator {
//     type Item = String;
//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.properties.is_empty() {
//             Some(self.properties.remove(0))
//         } else {
//             None
//         }
//     }
// }

impl IntoIterator for Book {
    type Item = String;
    // type IntoIter = BookIterator;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // BookIterator {
        //     properties: vec![self.title, self.author, self.genre]
        // }

        vec![self.title, self.author, self.genre].into_iter()
    }
}

fn main() {
    let book = Book {
        title: "Digital Image Proceessing".to_string(),
        author: "Gonzales".to_string(),
        genre: "Science Book".to_string(),
    };

    let book_iterator = book.into_iter();
    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());

    // Do the same as code above but better.
    for book_info in book_iterator {
        println!("{book_info}");
    }
}