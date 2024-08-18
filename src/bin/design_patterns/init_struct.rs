// --------------------------------------------
//          Inittializing Struct Instance
// --------------------------------------------

use basic_advance::modules::Student;

fn main() {
    // let std_1 = Student {
    //     id: 11,
    //     age: 20,
    //     name: "Joseph".to_string(),
    // };

    let std_1 = Student::new("Joseph".to_string());
    println!("{:?}", std_1);

    let std_2 = Student::new("Joseph".to_string()).unwrap_or_default();
    println!("{:?}", std_2);

    let std_3 = Student::new("joseph123".to_string()).unwrap_or_default();
    println!("{:?}", std_3);

    let std_4 = Student::new("joseph".to_string()).unwrap_or_default();
    println!("{:?}", std_4);

    let std_5 = Student{
        age: 12,
        ..Default::default()
    };
}