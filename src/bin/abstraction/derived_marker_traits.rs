// --------------------------------------------
//          Derived & Marker Traits
// --------------------------------------------

/*
    The derived will implement the trait for given struct 
    with basic default sort of implement prevent us 
    from implement the trait manually.

    - Derived Traits are available for common behaviors such as 
    "comparisons", "cloning" and "default" that initializing instances 
    of structs from some default values.

    - Marker Traits is typically used to add a metadata or constraints on a type
    to the compiler without requiring any actual functionality to be implemented.
    This is done by adding some super traits to the marker trait.

*/

// #[derive(Debug, PartialEq)]
#[derive(Debug, PartialEq, Default, Clone)]
struct Student {
    name: String,
    age: u8,
    sex: char,
}

impl Properties for Student {}

// Marker Traits enforce Student extend any traits the same as Properties(PartialEq, Default, Clone). 
trait Properties: PartialEq + Default + Clone {}

fn main(){
    let s_1 = Student {
        name: "ABC".to_string(),
        age: 35,
        sex: 'M',
    };

    let s_2 = Student {
        name: "XYZ".to_string(),
        age: 40,
        sex: 'M',
    };

    /*
        {:?} on println function is used to print out 
        some type with debug formatting and only work
        if the type which we are printing implements the debug trait.
    */
    println!("{:?}", s_1);
    println!("s_1 nad s_2 are equal: {}", s_1 == s_2);
}