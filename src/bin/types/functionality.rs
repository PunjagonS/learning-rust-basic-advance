// --------------------------------------------
//          Structs and its Functionlity
// --------------------------------------------

/*
    Implementing Block is the function which are tie to any structs.
*/

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

// Implementing block for Car struct
impl Car {
    // Method to display car information
    fn display_car_info_v2(&self) {
        println!(
            "Owner: {}, Year: {}, Price: {}",
            self.owner, self.year, self.price
        );
    }

    // Method to refuel the car
    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    // Method to sell the car
    fn sell(self) -> Self {
        self
    }

    /*
       Associated function (static method) - Do not take self as input parameter
       and cannot called using the dot syntax. It is called using the :: syntax.
    */
    fn monthly_insurance() -> u32 {
        123
    }

    // Method to calculate selling price
    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance() // The way we call associated function (:: syntax)
    }

    /*
       Associated function to create new instance of Car.
       Be called as Constructor in other languages.
    */
    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name.to_string(),
            year: year,
            fuel_level: 0.0,
            price: 0,
        }
    }
}

fn display_car_info(car: &Car) {
    println!(
        "Owner: {}, Year: {}, Price: {}",
        car.owner, car.year, car.price
    );
}

fn main() {
    let mut my_car = Car {
        owner: "ABC".to_string(),
        year: 2010,
        fuel_level: 0.0,
        price: 5_000,
    };

    // Old one
    display_car_info(&my_car);

    // Improve way
    my_car.display_car_info_v2();
    let new_owner = my_car.sell();
    //my_car.refuel(10.5); // Error: as we sell the car and it transfer the ownership

    let new_car2 = Car::new("XYZ".to_string(), 2020);
}
