// --------------------------------------------
//          Option
// --------------------------------------------

use rayon::vec;

#[derive(Debug)]
struct Student {
    name: String,
    grade: u32,
}

#[derive(Debug)]
struct Student2 {
    name: String,
    grade: Option<u32>,
}

fn get_grade(student_name: &String, student_db: &Vec<Student2>) -> Option<u32> {
    for student in student_db {
        if student.name == *student_name {
            return student.grade;
        }
    }
    None
}

fn main() {
    let student_db = vec![
        Student {
            name: "Alice".to_string(),
            grade: 90,
        },
        Student {
            name: "Charlie".to_string(),
            //grade, // Error: grade not defined
            grade: 0, // Not efficient if we want to make it clear that the grade is no value
        },
    ];

    let student_db2 = vec![
        Student2 {
            name: "Alice".to_string(),
            grade: Some(90),
        },
        Student2 {
            name: "Charlie".to_string(),
            grade: None,
        },
    ];

    let student_name = "Alice".to_string();
    let student_grade = get_grade(&student_name, &student_db2);

    match student_grade {
        Some(grade) => println!("{}'s grade is {}", student_name, grade),
        None => println!("{}'s grade is not available", student_name),
    }

    // Using if for more precise way and be single pattern
    if let Some(grade) = student_grade {
        println!("{}'s grade is {}", student_name, grade);
    } else {
        println!("{}'s grade is not available", student_name);
    }
}
