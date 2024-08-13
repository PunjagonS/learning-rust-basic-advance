// --------------------------------------------
//          Iterating Through Collections
// --------------------------------------------

use std::collections::HashMap;

fn main() {
    let vec_1 = vec![45, 30, 85, 90, 41, 39];
    // let mut vec_1_iter = vec_1.into_iter();
    // let value_1 = vec_1_iter.next();

    for values in &vec_1 {
        println!("{values}");
    }
    println!("{:?}", vec_1);

    // Try using hashmap instead of vec
    let mut persons: HashMap<String, i32> = HashMap::new();
    persons.insert("Hannash".to_string(), 40);
    persons.insert("Joseph".to_string(), 44);
    persons.insert("Sara".to_string(), 55);

    for (name, age) in persons {
        println!("This person {} has an age of {}", name, age);
    }
}