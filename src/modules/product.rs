pub use category::Category;                       // "use" declaration bringing item into scope

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    //category: category::Category,           // no need "crate" keyword for reference struct from another module
    category: Category,  

    // pub id: u64,
    // pub name: String,
    // pub price: f64,
    // pub category: Category,          
}
/*  
    Implement fuction for new instance of struct instead of make fields public
 */
impl Product {
    pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
        Product {
            // id: id,
            // name: name,
            // price: price,
            // category: category,
            id,
            name,
            price,
            category
        }
    }
}

mod category{
    #[derive(Debug, Clone, PartialEq)]
    pub enum Category {
        Electronics,
        Clothing,
        Books,
    }
}

impl Product {
    fn calculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }
}