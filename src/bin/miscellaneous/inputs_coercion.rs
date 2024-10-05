// --------------------------------------------
//        Function Inputs and Coercion
// --------------------------------------------

// fn vowels(words: &String) -> u8 { // Original
fn vowels(words: &str) -> u8 {
    let vowels_count = words
        .chars()
        .filter(|c| (*c == 'a') || (*c == 'e') || (*c == 'i') || (*c == 'o') || (*c == 'u'))
        .count();
    vowels_count as u8
}

// &Box[T] -> &T
// fn length_str(x: &Box<&str>) { // Original
fn length_str(x: &str) {
    println!("Length of the string: is {} is {}", x, x.len());
}

// &vec<T> -> &[T]
// fn square_values(num_vec: &Vec<i32>) { // Original
fn square_values(num_vec: &[i32]) {
    for i in num_vec {
        println!("The square is {}", i * i);
    }
}

fn main() {
    // Example 1
    let affan = "affan".to_string();

    // A reference to string could be coerced to a string slice.
    println!("{}: {:?}", affan, vowels(&affan));
    println!("Ferris: {}", vowels("Ferris"));

    // Example 2
    let box_str = Box::new("Hello");

    // A reference to a boxed string could be coerced to a string slice.
    length_str(&box_str);
    length_str("Hello Rust");

    // Example 3
    let values_vec = vec![1, 2, 3, 6, 5];
    let values_array = [1, 2, 3, 4, 5];

    // A reference to a vector could be coerced to a array.
    square_values(&values_vec);
    square_values(&values_array);
}
