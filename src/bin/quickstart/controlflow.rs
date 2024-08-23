// --------------------------------------------
//         - Loops
//         - For loop
//         - While loop
// --------------------------------------------

fn main() {
    // loop {
    //     println!("Simple loop");
    //     break;
    // }

    // // Named loop for breaking outer loop
    // 'outer: loop {
    //     'inner: loop {
    //         println!("Simple nested loop");
    //         //break 'inner; // break inner loop but continue outer loop
    //         break 'outer;
    //     }
    // }

    // // Loop expression with return value 10 but not recommended and practical use
    // let a = loop {
    //     break 10;
    // };

    let vec = vec![1, 2, 3, 4, 5];

    for i in vec {
        println!("For loop: {i}");
    }

    let mut num = 0;
    while num < 10 {
        println!("While loop: {num}");
        num += 1;
    }
}
