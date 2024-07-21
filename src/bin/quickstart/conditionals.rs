// --------------------------------------------
//         Conditionals
//         - If else
//         - If else if ladder
//         - Match
// --------------------------------------------

fn main() {
    let num = 40;
    if num < 50 {
        println!("Number is less than 50");
    } else {
        println!("Number is greater than or equal 50");
    }

    let marks = 95;
    let mut grade;

    // Using if else
    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }
    println!("Grade term 1 is {}", grade);

    // Using if else if ladder
    let marks = 70;
    grade = if marks >= 90 {
        // "A" // return type mismatch to others
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else if marks >= 60 {
        'D'
    } else {
        'F'
    };
    println!("Grade term 2 is {}", grade);

    // Using match
    let marks = 50;
    grade = match marks {
        90..=100 => 'A', // Arm 1 range
        80..=89 => 'B',  // Arm 2 range
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F', // Default Arm
    };
    println!("Grade term 3 is {}", grade);
}
