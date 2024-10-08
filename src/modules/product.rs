pub use category::Category;                       // "use" declaration bringing item into scope

/// Struct for storing product related information.
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    id: u64,
    pub name: String,
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
    /// # Example
    /// ```
    /// use basic_advance::modules::Product;
    /// use basic_advance::modules::Category;
    /// let some_product = Product::new(1, "Laptop".to_string(), 799.99, Category::Electronics);
    /// assert_eq!(some_product.name, "Laptop".to_string());
    /// ```
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
    /// Enum for representing the product categories.
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