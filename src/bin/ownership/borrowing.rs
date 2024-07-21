// --------------------------------------------
//          Borrowing
// --------------------------------------------
/*
- Borrowing Rules
    - At any given time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.

- Solve out two problem
    - Data Races
    - Dangling References
 */

fn main() {
    let mut vec_1 = vec![1, 2, 3, 4, 5];
    let ref1 = &mut vec_1;
    let ref2 = &mut vec_1;
    //println!("ref1: {:?}, ref2: {:?}", ref1, ref2); // Error: cannot borrow `vec_1` as mutable more than once at a time

    let ref3 = &vec_1;
    let ref4 = &vec_1;
    let ref5 = &vec_1;
    let ref6 = &vec_1;
    let ref7 = &vec_1;
    let ref8 = &vec_1;
    let ref9 = &vec_1;
    //let ref5 = &mut vec_1; // Error: cannot borrow `vec_1` as mutable because it is also borrowed as immutable
    //println!("ref3: {:?}, ref4: {:?}, ref5: {:?}", ref3, ref4, ref5);
    print!(
        "ref3: {:?}, ref4: {:?}, ref5: {:?}, ref6: {:?}, ref7: {:?}, ref8: {:?}, ref9: {:?}",
        ref3, ref4, ref5, ref6, ref7, ref8, ref9
    ); // Valid borrow as immutable references

    let vec_2 = {
        let vec_3 = vec![1, 2, 3, 4, 5];
        //&vec_3 // Error: `vec_3` reference is cleaned up after end scope(dangling reference). this will violation of rule 2
        vec_3
    };
}
