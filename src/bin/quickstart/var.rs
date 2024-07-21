// --------------------------------------------
//      - Variables
//          - Definition
//          - Mutability
//          - Scope
//          - Shadowing
//      - Constants
// --------------------------------------------

fn main() {
    // Definition
    let x: i8 = 120;
    println!("x is: {x}");

    // Immutability
    let _x = 5;
    //x = 10;

    // Mutability
    let mut _y = 5;
    _y = 10;

    // Scope
    {
        let _z = 15;
    }
    //let s = z;

    // Shadowing
    let t = 10;
    let t = t + 10;
    println!("t is: {t}");

    let _u = 3;
    let _u = 3.0;

    let v = 30;
    {
        let v = 40;
        println!("Inner v is: {v}");
    }
    println!("v is: {v}");

    // Constants
    const MAX_POINTS: u32 = 100_000; // Equal to 100,000
    print!("MAX_POINTS is: {MAX_POINTS}")
}
