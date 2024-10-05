// --------------------------------------------
//        Efficient Programming Skills
// --------------------------------------------

// 1. Avoid unnecessary copying of data
// 2. Use iterators to process data efficiently
// 3. Minimize the use of mutable state
// 4. Use efficient data structures
// 5. Avoid unnecessary allocations
// 6. Use efficient algorithms

use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn persons_by_name(person: Vec<Person>) -> HashMap<String, Person> {
    person.into_iter().map(|p| (p.name.clone(), p)).collect()
}

fn main() {
    // Example 1
    let cancer = true;
    let smoking = false;

    // original
    // match cancer {
    //     true => match smoking {
    //         true => println!("Your cancer is likely due to smoking"),
    //         false => println!(
    //             "Your cancer is not due to smoking and therefore it may have some other reasons"
    //         ),
    //     },
    //     false => match smoking {
    //         true => println!("Smoking is dangerous and may cause cancer"),
    //         false => {
    //             println!("You do not have any disease")
    //         }
    //     },
    // }

    // improved match logic
    match (cancer, smoking) {
        (true, true) => println!("Your cancer is likely due to smoking"),
        (true, false) => println!(
            "Your cancer is not due to smoking and therefore it may have some other reasons"
        ),
        (false, true) => println!("Smoking is dangerous and may cause cancer"),
        (false, false) => println!("You do not have any disease"),
    }

    /*
        Example 2 - Using collect on into_iter to get first error from server
        capturing OK variant by the Result.
    */
    let responses = vec![Ok(100), Err("Client Error"), Ok(300), Err("Server Error")];
    let result = responses.into_iter().collect::<Result<Vec<_>, &str>>();
    println!("{:?}", result);

    // Example 3 - Using HashMap to store persons by name
    let person_1 = Person {
        name: "Joseph".to_string(),
        age: 40,
    };
    let person_2 = Person {
        name: "Micheal".to_string(),
        age: 30,
    };

    let person_3 = Person {
        name: "Alexander".to_string(),
        age: 45,
    };

    let persons = vec![person_1, person_2, person_3];
    let person_hash = persons_by_name(persons);
    for (name, details) in person_hash {
        println!("Person: {:?}, has the details of {:?}", name, details);
    }

    // Example 4 - Using rev() to reverse the range for loop
    // for i in 9..0 {  // This will cause a compile-time error
    // for i in 0..=10 {
    for i in (0..=10).rev() {
        println!("{}", i);
    }
}
