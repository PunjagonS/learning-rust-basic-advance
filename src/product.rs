use category::Category;                       // "use" declaration bringing item into scope

    pub struct Product {
        id: u64,
        name: String,
        price: f64,
        //category: category::Category,           // no need "crate" keyword for reference struct from another module
        category: Category,           
    }
    
    // mod category{
    //     pub enum Category {
    //         Electronics,
    //         Clothing,
    //         Books,
    //     }
    // }

    mod category;

    impl Product {
        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }

        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }
    }