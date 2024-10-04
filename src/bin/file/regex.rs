// --------------------------------------------
//          Regular Expression Basics
// --------------------------------------------

use regex::Regex;

fn main() {
    // Condition match 1
    println!("---------------- Cond 1 -> [prt]ain ----------------");
    let regex = Regex::new(r"[prt]ain").unwrap();
    let text = "rrrain spain none";

    println!("The text has a match: {:?}", regex.is_match(text));
    println!("The text has a match: {:?}", regex.find(text)); // Search for the first match

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 2
    println!("---------------- Cond 2 -> [prt]..ain ----------------");
    let regex = Regex::new(r"[prt]..ain").unwrap(); // .. is any two characters
    let text = "rrrain spain none";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 3
    println!("---------------- Cond 3 -> gr[ae]y ----------------");
    let regex = Regex::new(r"gr[ae]y").unwrap();
    let text = "gray, grey, graye, grawy";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 4
    println!("---------------- Cond 4 -> [a-zA-Z]ain ----------------");
    let regex = Regex::new(r"[a-zA-Z]ain").unwrap(); // any lowercase or uppercase letter
    let text = "main pain tain rain but not 0ain";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 5
    println!("---------------- Cond 5 -> [^a-z]ain ----------------");
    let regex = Regex::new(r"[^a-z]ain").unwrap(); // ^ inside [] means not leading with a to z
    let text = "main pain tain rain but not 0ain";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 6
    println!(r"---------------- Cond 6 -> \d..... ----------------");
    // let regex = Regex::new(r"\d\d\d\d\d\d").unwrap(); // \d is any digit
    let regex = Regex::new(r"\d.....").unwrap(); // Shorthand for \d\d\d\d\d\d
    let text = "My phone number is 816030 and the second phone number is 816694 but my room number is A103005";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 7
    println!(r"---------------- Cond 7 -> ^aba ----------------");
    /*
        ^ outside square brackets [] means start with.
        In this case will match aba at the beginning of the text only.
    */
    let regex = Regex::new(r"^aba").unwrap();
    // let text = "ba abaa bc"; // will not match
    let text = "aba abaa bc"; // will match

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 8
    println!(r"---------------- Cond 8 -> bc$ ----------------");
    let regex = Regex::new(r"bc$").unwrap(); // $ means end with
    let text = "aba abaa bc";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 9
    println!(r"---------------- Cond 9 -> ^bc$ ----------------");
    let regex = Regex::new(r"^bc$").unwrap(); // ^ means start with and $ means end with
    let text = "bc";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 10
    println!(r"---------------- Cond 10 -> ^\d\d$ ----------------");
    let regex = Regex::new(r"^\d\d$").unwrap(); // ^\d\d$ means start with any digit, followed by any digit, and end with any digit.
    let text = "89";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 11
    println!(r"---------------- Cond 11 -> \b\w ----------------");
    /*
        \b is a word boundary(space, comma, etc.) and
        \w is any word character (letter, digit, or underscore).
    */
    let regex = Regex::new(r"\b\w").unwrap(); // Match any character at the beginning of a word
    let text = "Hi my name is nouman";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 12
    println!(r"---------------- Cond 12 -> \b\w* ----------------");
    let regex = Regex::new(r"\b\w*").unwrap(); // Match any word for seperate with word boundary
    let text = "Hi my name is nouman";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }
}
