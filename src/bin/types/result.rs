// --------------------------------------------
//          Result
// --------------------------------------------

use std::f32::consts::E;

use rayon::vec;

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
    None // Not reachable because we are checking the student before calling this function
}

/*
Result is a standard library enum that is used to represent the result of a computation that may fail.
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
fn check_student(student_name: &String, student_db: &Vec<Student2>) -> Result<(), String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(());
        }
    }
    Err("Student not found".to_string())
}

fn check_student_get_grade(
    student_name: &String,
    student_db: &Vec<Student2>,
) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err("Student not found".to_string())
}

fn main() {
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
    let student_status = check_student(&student_name, &student_db2);
    match student_status {
        Ok(_) => {
            let student_grade = get_grade(&student_name, &student_db2);
            if let Some(grade) = student_grade {
                println!("{student_name}'s grade is {grade}");
            } else {
                println!("{}'s grade is not available", student_name);
            }
        }
        Err(error_message) => {
            println!("Error: {}", error_message);
        }
    }

    let student_name = "Adam".to_string();
    // Using check_student_get_grade as implemented above
    let student_status = check_student_get_grade(&student_name, &student_db2);
    match student_status {
        Ok(option_grade) => {
            if let Some(grade) = option_grade {
                println!("{student_name}'s grade is {grade}");
            } else {
                println!("{}'s grade is not available", student_name);
            }
        }
        Err(error_message) => {
            println!("Error: {}", error_message);
        }
    }
}
