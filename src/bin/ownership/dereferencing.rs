// --------------------------------------------
//          Dereferencing
// --------------------------------------------

/*
    Dereferencing is the process of accessing the value
    that a reference points to. It is done using the `*` operator.
*/

fn main() {
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let deref_copy = *ref_1; // Copy the value of ref_1
    *ref_1 = 24; // Change the value of original `mut some_data`

    println!("some_data: {}", some_data);

    let mut heap_data = vec![1, 2, 3, 4, 5];
    let ref_1 = &mut heap_data;
    //let deref_copy = *ref_1; // Error: cannot move out of dereference of `&mut`-pointer
    let deref_copy = ref_1.clone(); // Clone the value of ref_1

    let move_out = ref_1;
    //let move_out_again = ref_1; // Error: follow to Borrowing rules that only one mutable reference is allowed
}
