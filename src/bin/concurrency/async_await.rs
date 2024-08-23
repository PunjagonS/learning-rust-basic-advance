// --------------------------------------------
//                Async Await
// --------------------------------------------

/*
    "Async" keyword make a function asynchronous and return
    something which implement the future trait with an assiciated type.

    The Future are like promises, which is giving you the promise
    that the function will generate some value in the future but don't know when.

    The Future is lazy and it's easy to cancel too.
*/

async fn printing() {
    println!("I am async function");
}

/*
    Add attribute: tokio macro allows main to be async and
    specifies that the async code will be executed by tokio runtime.
*/
#[tokio::main]
// fn main() {
async fn main() {
    // let x = printing();        // Return unit value (no output)
    let x = printing().await; // Wait the future be completion.

    let x = printing();
    println!("The future has not been polled yet.");
    // x.await;
    drop(x); // Not allowing to be polled in the remaining of the code.
}
