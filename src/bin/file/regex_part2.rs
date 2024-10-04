// --------------------------------------------
//    Regexes - Repeatitions and Quantifiers
// --------------------------------------------

use regex::Regex;

fn main() {
    // Condition match 1
    println!("---------------- Cond 1 -> a?aa ----------------");
    let regex = Regex::new(r"a?aa").unwrap(); // a? -> lead with a is optional and follow with aa
    let text = "aa aaa ba";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 2
    println!("---------------- Cond 2 -> ba? ----------------");
    let regex = Regex::new(r"ba?").unwrap(); // must lead with b and follow with a is optional
    let text = "a ba b baa";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 3
    println!(r"---------------- Cond 3 -> \w?\w?\w?.rs ----------------");
    let regex = Regex::new(r"\w?\w?\w?.rs").unwrap(); // must lead with 3 word and end with .rs
    let text = "fil.rs t1.rs file.rs";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 4
    println!(r"---------------- Cond 4 -> a+ ----------------");
    let regex = Regex::new(r"a+").unwrap(); // get all a
    let text = "a aa aaa baab bab";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 5
    println!(r"---------------- Cond 5 -> \w+\.gif ----------------");
    let regex = Regex::new(r"\w+\.gif").unwrap(); // get all word end with .gif
    let text = "image1.gif and background.gif";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 6
    println!("---------------- Cond 6 -> ab* ----------------");
    let regex = Regex::new(r"ab*").unwrap(); // get all a and optional follow with b or more b
    let text = "a ab abbbbb";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 7
    println!(r"---------------- Cond 7 -> \w{{3,5}} ----------------");
    let regex = Regex::new(r"\w{3,5}").unwrap(); // get all word with 3 to 5 character
    let text = "Hello i think you are happy because i have a gift for you";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 8
    println!(r"---------------- Cond 8 -> \b\d{{1,3}}\.\d{{1,3}}\b ----------------");
    let regex = Regex::new(r"\b\d{1,3}\.\d{1,3}\b").unwrap(); // get all number with 1 to 3 character and . with 1 to 3 character
    let text = "921.583 0.0 1456.25";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 9
    println!(r"---------------- Cond 9 -> \d{{3,}} ----------------");
    let regex = Regex::new(r"\d{3,}").unwrap(); // get all number with 3 or more character
    let text = "5321 500 5698 12";

    for cap in regex.captures_iter(text) {
        println!("Found match: {:?}", &cap[0]);
    }

    // Condition match 10 (Grouping)
    println!(r"---------------- Cond 10 -> (\d{{4}})-(\d{{2}})-(\d{{2}}) ----------------");
    let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap(); // get date with format (YYYY-MM-DD)
    let text = "2012-03-14 2013-01-01 2014-07-05";

    for cap in regex.captures_iter(text) {
        println!(
            "Month: {} Day: {} Year: {}, the whole: {}",
            &cap[2], &cap[3], &cap[1], &cap[0]
        );
    }
}
