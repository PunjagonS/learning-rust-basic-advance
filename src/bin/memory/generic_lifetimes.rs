// --------------------------------------------
//          Generic Lifetimes
// --------------------------------------------

/*
    Lifetime Specifiers are also known as Generic Lifetime annotation ('a).

    Purpose to guide compiler know that the lifetime of 
    return reference will be equal to shortest lifetime of 
    the input reference.

    When to Use Generic Lifetimes
    1. Functions or Methods with Borrowed Parameters:
        If a function or method accepts references and returns a reference, and the lifetime 
        of the returned reference depends on the lifetimes of the input references, you should use a generic lifetime.
        This allows the function to work with references of any lifetime as long as they are compatible.

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

    2. Structs with References:
        If a struct holds references, you use generic lifetimes to ensure the 
        struct's references are valid for as long as the struct exists.

        struct ImportantExcerpt<'a>

    3. Traits with References:
        When defining traits that work with references, you might use generic lifetimes to allow 
        the trait to be implemented for types that have different lifetimes.

        trait Summary<'a>

    4. Functions or Methods with Multiple References:
        When a function or method has multiple references and you want to express relationships 
        between their lifetimes, you use generic lifetimes.

        fn first_word<'a>(s: &'a str) -> &'a str
*/

fn main() {

    /*
        Example 1 : define int1 and int2 to have the same lifetime till end of main scope.
    */
    let int1 = 5;
    let int2 = 10;
    let picked_value = picking_int(&int1, &int2);
    println!("{picked_value}");

    // let picked_value = picking_int_2(&int1, &int2);
    let picked_value = picking_int_3(&int1, &int2);
    println!("{picked_value}");


    /*
        Example 2 : modify the code so that 1 of 2 inputs('int3' or 'int4')
        has a shorter lifetime. In this case 'int4' lifetime is shorter and 
        valid only within code block but 'int3' will be valid till main() end.
    */
    let int3 = 3;
    {
        let int4 = 4;
        let picked_value = picking_int_3(&int3, &int4);
        println!("{picked_value}");
    }


    let int3 = 3;
    let picked_value;
    let picked_value_2;
    let picked_value_3;
    {
        let int4 = 4;

        /*
            Error: `int4` does not live long enough to be referenced(borrowed) 
            to 'picked_value' to print outside the code block.

            More specifically, it will be considered a "Dangling Reference".
        */
        // picked_value = picking_int_3(&int3, &int4);

        /*
            Valid: cause 'picking_int_4' returning reference has a lifetime 
            equal to the first parameter, which is '&int3' that live long till main() end.
        */
        picked_value = picking_int_4(&int3, &int4);


        /*
            Static Lifetime is the special lifetime reference that live long equal to the entire duration of program.

            Example : 'picking_int_5' and 'picking_int_6' returning Static Lifetime reference.
            but 'picking_int_5' is bind returning lifetime to equal first parameter '&int3'(not satified).
        */
        picked_value_2 = picking_int_5(&int3, &int4);
        picked_value_3 = picking_int_6(&int3, &int4);
    }
    println!("{picked_value}");
    println!("{picked_value_2}");
    println!("{picked_value_3}");
}

// Original before use generic lifetimes
fn picking_int(i: &i32, j: &i32) -> i32 {
    if rand::random() {
        *i                                      // Dereference to return i32 primitive value
    } else {
        *j                                      // Dereference to return i32 primitive value
    }
}

/*
    Error: missing lifetime specifiers because compiler needs to know how long the references i and j 
    (which are borrowed from somewhere) will be valid. Since i and j can have a difference lifetime
    before passed as input to function.

    When a function returns a reference, compiler requires lifetime annotations('a) to ensure that 
    the returned reference is valid for the expected duration, preventing potential dangling references.
*/ 
// Error: lifetime specifier missing
// fn picking_int_2(i: &i32, j: &i32) -> &i32 {
//     if rand::random() {
//         i
//     } else {
//         j
//     }
// }


/*
    Solution: add generic lifetime annotation ('a).
    - '<'a>' introduces a lifetime parameter 'a.
    - '&'a i32' specifies that both i and j have the same lifetime 'a, meaning they are valid for the same duration.
    - '-> &'a i32' means that the returned reference will have the same lifetime 'a.
    This tells the compiler that the returned reference will be valid as long as both i and j are valid, which resolves the issue.

    For instance below if 'i' is found to have a shorter lifetime of the 2 variables,
    then the returning value wll have the lifetime equal to 'i', and if 'j' has a shorter lifetime than i,
    then the returning value will have a lifetime equal to that of 'j'.
*/
fn picking_int_3<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if rand::random() {
        i
    } else {
        j
    }
}


fn picking_int_4<'a>(i: &'a i32, j: &i32) -> &'a i32 {
        i
}

/*
    Return local Static Lifetime reference 'y' but the function signature 
    is a bit confusing in this case, since 'y' lifetime will be
    equal to first parameter only.
*/
fn picking_int_5<'a>(i: &'a i32, j: &i32) -> &'a i32 {
    let y: &'static i32 = &6;
    y
}

/*
    Remove the generic lifetime annotations. instead that
    we will return a reference with a static lifetime('static' keyword).
*/
fn picking_int_6(i: &i32, j: &i32) -> &'static i32 {
    let y: &'static i32 = &6;
    y
}