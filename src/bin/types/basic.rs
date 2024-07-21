// --------------------------------------------
//          Structs and its Types
// --------------------------------------------

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

fn main() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        price: 5_000,
    };

    let car_year = my_car.year;
    my_car.fuel_level = 30.0;
    //let extracted_owner = my_car.owner; // Transfer owner ship
    let extracted_owner = my_car.owner.clone(); // Transfer owner ship
    println!("Owner is: {}", my_car.owner);

    let another_car = Car {
        owner: "new_name".to_string(),
        ..my_car // Copy remains my_car field to another_car
    };
    println!("Owner is: {}", my_car.owner);

    let third_car = Car { ..my_car };
    //println!("Owner is: {}", my_car.owner); // Error cause my_car.owner transfer owner ship

    // Tuple
    let point1: (i32, i32) = (1, 3);
    let point2: (i32, i32, i32) = (4, 10, 13);

    // Tuple Structs
    struct Point2d(i32, i32);
    struct Point3d(i32, i32, i32);

    let point1 = Point2d(1, 3);
    let point2 = Point3d(4, 10, 13);

    // Unit Struct
    struct ABC;
}
