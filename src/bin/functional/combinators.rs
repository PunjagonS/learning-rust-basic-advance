// --------------------------------------------
//         Combinators
// --------------------------------------------

/*

*/

fn main() {
    let words = vec!["apple", "banana", "grap", "orange", "pear"];
    let mut result: Vec<String> = vec![];

        for word in words {
            if word.starts_with('a') || word.starts_with('b'){
                let uppercase_word = word.to_uppercase();
                result.push(uppercase_word);
            }
        }
        println!("Result: {:?}", result);
}