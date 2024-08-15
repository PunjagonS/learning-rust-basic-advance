// --------------------------------------------
//          Lifetimes in Structs
// --------------------------------------------

/*
    Example show struct fields may contain references.
    
    Lifetime Elision are not defined for structs cause
    we tend to write functions that uses references
    far more often than we tend to write data structures
    that use references.
*/

struct ArrayProcessor<'a>{
    data: &'a [i32]
}

impl<'a> ArrayProcessor<'a> {
    /*
        This is the original function signature with Lifetime Elision.
        -> fn update_data(&mut self, new_data: &'a [i32]) -> &[i32] <-

        Code below is expanded by the compiler resolved to the final form based on lifetime Elision Rules.
    */
    fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &'b [i32] { 
        let previous_data = self.data;
        self.data = new_data;
        &previous_data
    }
}

fn main() {
    let mut some_data = ArrayProcessor { data: &[4, 5, 6] };

    let previous_data = some_data.update_data(&[5, 8, 10]);
    println!("Previous data: {:?}", previous_data);
    println!("New data: {:?}", some_data.data);
}