// --------------------------------------------
//                  Todo Macros
// --------------------------------------------

#[derive(Default)]
struct Student {
    name: String,
    age: u8,
    sex: char,
    country: String,
    salary: u16,
    nationality: String,
}

impl Student {
    fn some_fn_1(&self) -> String {
        // todo!() // Defer the implementation of this function
        "".to_string()
    }

    fn some_fn_2(&self) -> u8 {
        todo!() // Defer the implementation of this function
    }
}

trait GeneralInfo {
    fn get_info(&self) -> (&str, u8, char);
    fn get_country(&self) -> &str;
}

impl GeneralInfo for Student {
    fn get_info(&self) -> (&str, u8, char) {
        todo!()
    }

    fn get_country(&self) -> &str {
        todo!()
    }
}

fn main() {
    let student = Student::default();
    student.some_fn_1();
}
