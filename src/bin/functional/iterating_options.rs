// --------------------------------------------
//          Iterating Through Options
// --------------------------------------------

/*
    Options type is implemented IntoIterator.
*/

fn main() {
    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "charger"];

    // This code work but could be simplified using the if syntax.
    match some_product {
        Some(product) => products.push(product),         // Match when some_product is not none
        _ => {}                                          // Match if some_product is none and do nothing
    };

    // This code work but could be simplified using the extend method.
    if let Some(product) = some_product {                // "true" if some_product is not none
        products.push(product);
    }

    // Using extend method is to append contents of an iterator.
    products.extend(some_product);
    println!("{:?}", products);

    // Using chain() method to combine 2 iterators.
    let products_iter = products.iter().chain(some_product.iter());     // .chain() return combine of 2 iterators
    for prod in products_iter {
        println!("{:?}", prod);
    }

    // Another use cases
    let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];
    let mut prod_without_none = Vec::new();
    for p in &products {
        if p.is_some() {
            prod_without_none.push(p.unwrap());
        }
    }

    //// Work the same.
    // let prod_without_none = products
    // .into_iter()                                                         // Enforce take ownership.
    // .filter(|x| x.is_some())
    // .map(|x| x.unwrap())
    // .collect::<Vec<&str>>();

    // Using Flatten method to extract out the items stored inside the some variant and throw away the none variant.
    let prod_without_none = products.into_iter().flatten().collect::<Vec<&str>>();

    println!("{:?}", prod_without_none);
}