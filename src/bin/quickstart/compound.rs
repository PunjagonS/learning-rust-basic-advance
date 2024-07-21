// --------------------------------------------
//      - Compound Data Types (can hold multiple values)
//          - &str and String
//          - Arrays
//          - Vectors
//          - Tuples
//          - Empty Tuple
// --------------------------------------------

fn main() {
    // &str(called slice)  ||||||  String(type)
    let fixed_str = "Fixed length string";
    // fixed_str = "Change the value of fixed_str"; // Error: cannot assign twice to immutable variable
    let mut flexible_str = String::from("Flexible length string");
    flexible_str = String::from("Change the value of flexible_str");

    // Arrays
    let mut array_1: [i32; 5] = [1, 2, 3, 4, 5];
    let num = array_1[3];

    println!("{:?}", array_1);
    let array_2 = [0; 10]; // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    // Vectors
    let vec_1: Vec<i32> = vec![4, 5, 6, 8, 9]; // Can be resized at runtime and there is no size limit
    let num = vec_1[3];

    // Tuples
    let my_info = ("", 40000, "Age", 25);
    let salary_value = my_info.1;
    let (salary, salary_value, age, age_value) = my_info;

    let unit = (); // Means nothing. for example, a function that returns nothing and not consume any memory
}
