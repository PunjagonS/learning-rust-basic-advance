// --------------------------------------------
//          Concrete Lifetimes
// --------------------------------------------

/*
    Concrete Lifetime is the time during which a value exists
    inside the memory.

    The lifetime of a value starts when it is created, 
    and ends when the value dropped or move out from 
    a particular memory location, mainly due to cahnge of ownership.

    Dangling Reference is a reference to a value in memory which does not exist.
*/

fn main() {

    //// Error from value i is dropped and cleared when inner scope end.
    // {
    //     let i = 5;
    // }
    // let j = i;
    // println!("i: {i}");

    let str_1 = "abc".to_string();
    str_fn(str_1);                                  // Ownership "abc" transfered to str_fn()
    
    // let str_2 = str_1;                           // Error cause str_1's value dropped and cleared.
    // println!("str_1 : {str_1}");                 // Error cause str_1's value dropped and cleared.

    
    let i: &i32;
    {
        let j = 5;
        i = &j;                                      
        println!("i : {i}");
    }
    // println!("i : {i}");                         // Error dangling reference cause i cannot reference to j outside scope.


    let mut vec_1 = vec![6,5,8,9];

    /*
        Follow to borrowing rule, we cannot have mutable && immutable reference at the same time

        The rust use another concept called non-lexical lifetimes.

        Non-lexical lifetimes aim to relax some of the strictness imposed
        by the typical lifetimes. By analyzing the actual usage of references in the code,
        rather than solely relying on scopes so the non-lexical lifetimes
        are lifetimes which are not tied to scope.
    */
    let ref_1 = &vec_1;                 // Immutable reference of vec_1
    // let ref_2 = &mut vec_1;                     // Error from try to declare mutable reference of vec_1
    println!("ref_1: {:?}", ref_1);
    
    /*
        In this case, the compiler notices that the ref_1 was created on the line 53
        and its last usage is on the next print line 55. Therefore, its lifetime is
        limited to those 2 lines above 53,55. In the same way compiler look into ref_2
        and notice that it's created on the line 63, and its useage is on 3 lines 67,68,69.
        Therefore, its lifetime is limited to these 3 lines.

        Summary : the lifetimes of the 2 references, ref_1 and ref_2 does not overlap.
        Therefore, they do not coexist. As a result, the borrowing rules are not violated.
    */
    let ref_2 = &mut vec_1;         // Mutable reference
    ref_2.push(3);
    println!("ref_2: {:?}", ref_2);
}

fn str_fn(s: String){
    println!("s : {s}");
}