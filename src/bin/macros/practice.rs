/*
    We want to create a macro called make_struct which will create a new struct containing some fields.
    The input to the macro is the name of the struct and the name of the fields along with their types.
    The skeleton of the macro along with its left sides of the rules are given.
    You are required to write the code for the expansion or the right side of the rule.
*/
macro_rules! make_person_struct {
    ($struct_name:ident {$($field:ident: $ty:ty),* }) => {
        struct $struct_name {
            employee_id: u32,
            first_name: String,
            middle_name: String,
            last_name: String,
            branch: String,
            $($field: $ty),*
        }
    };
}

make_person_struct!(Manager {
    squad_members: Vec<String>
});

make_person_struct!(Employee {
    boss_id: u32,
    boss_name: String,
    job_title: String,
    shift: Shift
});

enum Shift {
    Morning,
    Afternoon,
    Evening,
}

// Consider the code below. Show the expansion part of this code, espacially for the invocation to macro.
macro_rules! make_functions {
    ($($func_name:ident: $return_type:ty => $return_expr:expr),+) => {
        $(
            fn $func_name() -> $return_type {
                $return_expr
            }
        )+
    };
}
make_functions!(foo: i32 => 42, bar: String => "hello world".to_owned());

// Consider the code below. Write the expanded version of the code that can be viewed using the cargo expand utility.
macro_rules! sum_macro {
    ($($x:expr),*) => {
        {
            let mut sum = 0;
            $(sum += $x;)*
            sum
        }
    };
}

fn main() {
    // Practice 1
    let branch_manager = Manager {
        employee_id: 1,
        first_name: "John".to_string(),
        middle_name: "Doe".to_string(),
        last_name: "Smith".to_string(),
        branch: "London".to_string(),
        squad_members: vec!["John".to_string(), "Jane".to_string()],
    };

    let employee = Employee {
        employee_id: 2,
        first_name: "Jane".to_string(),
        middle_name: "Doe".to_string(),
        last_name: "Smith".to_string(),
        branch: "London".to_string(),
        boss_id: 1,
        boss_name: "John".to_string(),
        job_title: "Slave".to_string(),
        shift: Shift::Morning,
    };

    // Practice 2
    let result1 = foo();
    let result2 = bar();
    println!("foo result: {}", result1);
    println!("bar result: {}", result2);

    // Practice 3
    let result = sum_macro!(1, 2, 3, 4, 5);
    println!("sum result: {}", result);
}
