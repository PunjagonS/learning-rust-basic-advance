// --------------------------------------------
//          Iterator
// --------------------------------------------

/*
    The iterator design pattern allows a diffeerence typs 
    to have a common interface for accessing the elements
    of a type sequentially.
*/

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Employee {
    name: String,
    salary: u16,
}

struct Employee_Records {
    employee_db: Vec<Employee>,
}

impl Iterator for Employee_Records {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let emp_1 = Employee {
        name: String::from("John"),
        salary: 40_000,
    };

    let emp_2 = Employee {
        name: String::from("Joseph"),
        salary: 30_000,
    };

    let mut emp_db = Employee_Records {
        employee_db: vec![emp_1, emp_2],
    };

    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());

    /*
        The loop know how to handle the types that implement the iterators.
        The loop will automatically end when the none variant is encountered.
        Futhermore, all the values returned from the next() will be unwrapped.
    */
    for employee in emp_db {
        println!("{employee}");
    }
}