/*  -----------------------------------------------------
    Popularity SCores
        Description
        - Given some products along with its respectively popularity scores,
          We want to determine if the popularity is fluctuating, increasing, decreasing

        Tools
        - HashMaps
        - Loops
        - Conditional `if`
    -----------------------------------------------------
*/

use std::collections::HashMap;

fn popularity_analysis(scores: Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..scores.len() - 1 {
        if scores[i] > scores[i + 1] {
            increasing = false;
        }

        if scores[i] < scores[i + 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}

fn main() {
    let mut products = HashMap::new();

    products.insert("Product 1", vec![1, 2, 2, 3]);
    products.insert("Product 2", vec![4, 5, 6, 3, 4]);
    products.insert("Product 3", vec![8, 8, 7, 6, 5, 4, 4, 1]);

    /*
        The order is not enforced in the Hashmap so forloop will not get item
        from products in order.
    */
    for (product_id, popularity) in products {
        match popularity_analysis(popularity) {
            true => println!("{} popularity is increasing or decreasing", product_id),
            false => println!("{} popularity is fluctuating", product_id),
        }
    }
}
