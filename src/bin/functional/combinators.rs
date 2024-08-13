// --------------------------------------------
//         Combinators
// --------------------------------------------

/*
    Combinators is like method chaining result in lesser and more clean and clear code.
*/

fn main() {
    let words = vec!["apple", "banana", "grap", "orange", "pear"];
    let mut result: Vec<String> = vec![];

    //// Replace this code block with combinators
    // for word in words {
    //     if word.starts_with('a') || word.starts_with('b'){
    //         let uppercase_word = word.to_uppercase();
    //         result.push(uppercase_word);
    //     }
    // }
    // println!("Result: {:?}", result);

    // Using combinators syntax, work the same as code above 
    let result = words
    .into_iter()
    .filter(|&word| word.starts_with('a') || word.starts_with('b'))
    .map(|word| word.to_uppercase() )
    /*
        Alternative use Turbofish syntax
        .collect::<Vec<String>>();

        or set result type explixitly

        result: Vec<String>
    */
    .collect::<Vec<String>>();

    println!("Result: {:?}", result);
}