// --------------------------------------------
//         Performance Lints(Clippy)
// --------------------------------------------

struct A {
    // values: Box<Vec<i32>>,
    /*
        To avoid a using memory allocator on the heap,
        since heap management is typically being done using
        the operating system.
    */
    values: Vec<i32>,
}

enum B {
    Variant1(i32), // Auto allocates memory the same with largest Variant2.
    // Variant2([i32; 10_000]), // Array i32 with fixed size 10_000 elements.

    /*
        Improved by using box smart pointer.
        The box in this case will take a space equal to only size of the pointer
        (1 row stack storing 1 memory address), which is pointing to some resource on the heap,
        which is way too less than the previous size of 10_000 elements.
    */
    Variant2(Box<[i32; 10_000]>),
}

fn main() {
    // Example - Unecessary box(making new heap allocation)
    let x = Box::new(32u32);

    // Example - Boxing default with shorter syntax.
    let y: Box<i32> = Box::new(Default::default());
    let y: Box<i32> = Box::default(); // Same as above but shorter and more efficient from performance perspective.

    // Example - Compare string and string slice.
    let x = "Nouman".to_string();
    let y = "Nouman";
    /*
        The comparison operator can operate on references.

        Creating own value like below effectively throws it away directly
        afterward, which is needlessly consuming code and heap space.

        This means that the allocation is never assigned to anything
        and is only being used within above line and will be thrown away
        after the line below.
    */
    if x == y.to_owned() {
        println!("Both are same");
    }

    // More efficient and effective code.
    if x == y {
        println!("Both are same");
    }

    // Example - Extend and append vector.
    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5, 6];
    // Used this for doing some logic before adding elements to vector(includes iterator).
    a.extend(b.drain(..));

    // This is more efficient if only adding elements directly to vector.
    a.append(&mut b);
    ///////////////////////////////////////////////////////////////////////////////////////////////

    // Example - Replace string.
    let hello = "hesuo worpd"
        .replace("s", "l")
        .replace("u", "l")
        .replace("p", "l");

    // More efficient and effective code for replacing multiple characters.
    let hello = "hesuo worpd".replace(['s', 'u', 'p'], "l");
}
