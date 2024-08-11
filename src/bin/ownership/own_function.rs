// --------------------------------------------
//          Ownership Functions
// --------------------------------------------

fn main() {
    let vec_1 = vec![1, 2, 3, 4, 5];
    takes_ownership(vec_1);
    //println!("Vector: {:?}", vec_1); // Error: value borrowed here after move (vec_1 -> v in takes_ownership)

    let vec_2 = vec![6, 7, 8, 9, 10];
    takes_ownership(vec_2.clone());
    println!("Vector 2: {:?}", vec_2); // No error

    let vec_3 = gives_ownership();
    println!("Vector 3: {:?}", vec_3);

    let vec_4 = takes_and_gives_ownership(vec_3);
    //println!("Vector 3: {:?}", vec_3); // Error: value borrowed here after move (vec_3 -> v in takes_and_gives_ownership)
    println!("Vector 4: {:?}", vec_4);

    // Stack function with primitive type no effect on ownership cause it creates a copy
    let x = 10;
    stack_function(x);
    println!("In main, x is: {x}");
}

// Function takes ownership of the vector and drops it after the function call
fn takes_ownership(v: Vec<i32>) {
    println!("Vector: {:?}", v);
}

fn gives_ownership() -> Vec<i32> {
    vec![1, 2, 3, 4, 5] // Ownership of vec![1, 2, 3, 4, 5] is transferred to the one calling this function
}

fn takes_and_gives_ownership(mut v: Vec<i32>) -> Vec<i32> {
    v.push(6);
    v
}

// Function takes primitive type and does not take ownership but creates a copy
fn stack_function(mut var: i32) {
    var = 56;
    print!("In func, var is: {var}")
}
