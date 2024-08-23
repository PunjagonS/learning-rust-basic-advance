// --------------------------------------------
//              Hash Maps
// --------------------------------------------

use std::collections::HashMap;

fn main() {
    let mut person = HashMap::new();
    person.insert("Alice", 24);
    person.insert("Bob", 25);
    person.insert("Charlie", 26);

    println!("The age is {:?}", person.get("Alice").unwrap());

    if person.contains_key("Bob") {
        println!("Bob is present in the hashmap");
    } else {
        println!("Bob is not present in the hashmap");
    }

    match person.get("Charlie") {
        Some(value) => println!("The value exist {}", value),
        None => println!("The value does not exist"),
    }

    for (name, age) in &person {
        println!("The person {} is of age {}", name, age);
    }

    let mut likes = HashMap::new();
    likes.insert("Alice", "apple");
    likes.insert("Alice", "mango");
    likes.insert("Alice", "orange"); // This will overwrite the previous value if key is same
    println!("The fruit which is being liked is {:?}", likes);

    likes.entry("Alice").or_insert("banana"); // This will not overwrite the value if key existed
    println!("The fruit which is being liked is {:?}", likes);

    // Final Example
    let some_vec = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq = freq_vec.entry(*i).or_insert(0);
        println!("The frequency of {} is {}", i, freq);
        *freq += 1;
    }
    // Printing the frequency of each element such as 5 is 6, 8 is 2, 1 is 2, 0 is 1
    println!("{:?}", freq_vec);
}
