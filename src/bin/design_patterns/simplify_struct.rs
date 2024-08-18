// --------------------------------------------
//            Simplifying Structs
// --------------------------------------------

struct A {
    // f1: u32,
    // f2: u32,
    // f3: u32,

    b: B,
    c: C,
}

// Decomposition struct A
struct B {
    f2: u32,
}

struct C {
    f1: u32,
    f3: u32,
}

// Original root cause error in fn3.
// fn fn1(a: &mut A) -> &u32 {
//     &a.f2
// }

// Solution 1: remove mutable to immutable borrowing.
// fn fn1(a: &A) -> &u32 {
//     &a.f2
// }

// Solution 2: return copy of value instead of reference.
// fn fn1(a: &A) -> u32 {
//     a.f2
// }

// fn fn2(a: &mut A) -> u32 {
//     a.f1 + a.f3
// }

// Solution 3: decomposition to suitably borrow the struct seperately.
fn fn1(a: &mut B) -> u32 {
    a.f2
}

fn fn2(a: &mut C) -> u32 {
    a.f1 + a.f3
}

fn fn3(a: &mut A) {

    /*
        Error: fn2(a) borrow `*a` as mutable more than once at a time.

        Since the function using structure as a mutable borrow.
        Therefore, as long as the variable x is alive or in scope,
        the rust compiler assumes that the whole of the struct
        is being borrowed as MUTABLE!.

        In summary, the problem in this case is that the rust 
        does not allow the fields to be borrowed independently
        to make sure that the struct will not be updated inside
        the function.

        This means that borrowing the field enforces the borrowing
        of the whole struct.
    */
    // let x = fn1(a);
    // let y = fn2(a);
    // println!("{x}");


    // Using decomposition.
    let x = fn1(&mut a.b);
    let y = fn2(&mut a.c);
    println!("{x}");
}

fn main() {}