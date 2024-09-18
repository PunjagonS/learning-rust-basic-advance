/*  -----------------------------------------------------
    Suggesting items for Special Shopping Card
        Description
        - Given a list of prices, return a couple of items with
          their sum matching the gift card.

        Tools
        - Hashsets
        - Vectors
    -----------------------------------------------------
*/

use std::collections::HashSet;

fn products_suggestions(products: Vec<i32>, money: i32) -> Vec<Vec<i32>> {
    let mut prices_hash = HashSet::new();
    let mut offers = Vec::new();

    /*
        Loop through the products and calculate the difference for suggesting
        customer spend the rest of the money with other product.
    */
    for product in products {
        let possible_paired_product = money - product;
        if prices_hash.get(&possible_paired_product).is_none() {
            prices_hash.insert(product);
        } else {
            offers.push(vec![product, possible_paired_product]);
        }
    }

    offers
}
fn main() {
    let products = vec![11, 30, 55, 34, 45, 10, 19, 20, 60, 5, 23]; // Prices of products.
    let offers = products_suggestions(products, 50);
    println!("{:?}", offers);
}
