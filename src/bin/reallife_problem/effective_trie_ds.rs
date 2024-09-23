/*  -----------------------------------------------------
        Efficient Storage and Retrieval of Words
    -----------------------------------------------------
*/

use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct WordsDictionary {
    root: Node,
}

impl WordsDictionary {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for w in word.chars() {
            /*
                Moves current pointer to the child node if it exists,
                otherwise creates a new node with `w` and moves the pointer to it.
            */
            current = current.children.entry(w).or_default();
        }
        current.is_word = true; // Mark the end of the word at last node.
    }

    fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for w in word.chars() {
            match current.children.get(&w) {
                Some(node) => current = node,
                None => return false,
            }
        }
        current.is_word
    }
}

fn main() {
    let words = vec![
        "the", "a", "there", "answer", "any", "by", "bye", "their", "abc",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

    let mut words_dict = WordsDictionary::new();
    for word in words {
        words_dict.insert(&word);
    }

    println!(
        "Searching for 'there' in the dictionary result: {}",
        words_dict.search("there")
    );

    println!(
        "Searching for 'that' in the dictionary result: {}",
        words_dict.search("that")
    );
}
