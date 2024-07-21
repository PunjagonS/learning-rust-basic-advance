// --------------------------------------------
//          Borrowing in Functions
// --------------------------------------------

/*
- Borrowing Rules
    - At any given time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.
 */
fn main() {
    let vec_1 = vec![1, 2, 3, 4, 5];
    //takes_ownership(vec_1); // Transfer ownership to function
    //println!("Vector: {:?}", vec_1); // Error: value borrowed here after move

    takes_ownership(vec_1.clone()); // Not recommended, cause this will make new heap allocation and not efficient
    println!("vec 1: {:?}", vec_1); // No error

    let ref_1 = &vec_1;
    //let ref_2 = &mut vec_1; // Error: cannot borrow `vec_1` as mutable because it is also borrowed as immutable
    borrows_vec(&ref_1); // Borrowing reference
    println!("vec 1: {:?}", ref_1); // No error

    //takes_and_gives_ownership(vec_1);
    //println!("Vector 1: {:?}", vec_1); // Error: value borrowed here after move

    // Shadowing
    let vec_1 = takes_and_gives_ownership(vec_1); // Can compile but not not efficient
    println!("vec 1: {:?}", vec_1); // No error

    let mut vec_2 = vec![1, 2, 3, 4, 5];
    let ref_3 = &mut vec_2; // Borrowing mutable reference
    mutably_borrows_vec(ref_3);
    println!("vec 2: {:?}", ref_3); // No error
}

fn takes_ownership(v: Vec<i32>) {
    println!("vec: {:?}", v);
}

fn borrows_vec(v: &Vec<i32>) {
    println!("vec: {:?}", v);
}

fn takes_and_gives_ownership(mut v: Vec<i32>) -> Vec<i32> {
    v.push(6);
    v
}

fn mutably_borrows_vec(v: &mut Vec<i32>) {
    v.push(6);
}

// Bad example to try to return invalid reference for borrowing cause the vector is dropped after function called
// fn gives_ownership() -> &Vec<i32> {
//     let vec = vec![4, 5, 6, 7, 8];
//     &vec
// }
