// --------------------------------------------
//          Enums
// --------------------------------------------

/*
    An enumeration, also referred to as an enum, is a simultaneous definition of a nominal enumerated type as well as a set of constructors,
    that can be used to create or pattern-match values of the corresponding enumerated type.

    Enumerations are declared with the keyword enum.

    Enumeration constructors can have Tuple, Struct, Unit either named or unnamed fields:

    In this example, Cat is a struct-like enum variant, whereas Dog is simply called an enum variant 
    where Dog is Tuple and Cat is Struct
    enum Animal {
        Dog(String, f64),                       unnamed field (String, f64)
        Cat{ name: String, weight: f64 },       named
    }

    An enum where no constructors contain fields are called a field-less enum. For example, this is a fieldless enum:
    enum Fieldless {
        Tuple(),            - enum variant
        Struct{},           - struct-like enum variant
        Unit,               - unit variants
    }

    Structs and enums are powerful constructs on their own, but they can be even more powerful when combined. 
    By using enums to group related structs or adding a field to a struct that is an enum type, 
    you can create complex data types that accurately model your problem domain.

    enum Enemy {
        Goomba(Goomba),
        KoopaTroopa(KoopaTroopa),
        HammerBro(HammerBro),
    }

    struct Goomba {
        position: Position,
        is_stomped: bool,
    }

    struct KoopaTroopa {
        position: Position,
        is_stomped: bool,
        is_in_shell: bool,
    }
 */

#[derive(Debug)]
enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
enum TravelType {
    Car,
    Train,
    Aeroplane,
}

#[derive(Debug)]
enum TravelType2 {
    Car(f32),
    Train(f32),
    Aeroplane(f32),
}

impl TravelType {
    fn travel_allowance(&self, miles: f32) -> f32 {
        match self {
            TravelType::Car => miles * 0.5,
            TravelType::Train => miles * 0.3,
            TravelType::Aeroplane => miles * 0.8,
        }
    }
}

impl TravelType2 {
    fn travel_allowance_improved(&self) -> f32 {
        match self {
            TravelType2::Car(miles) => miles * 0.5,
            TravelType2::Train(miles) => miles * 0.3,
            TravelType2::Aeroplane(miles) => miles * 0.8,
        }
    }
}

fn main() {
    let mut day = "Saturday".to_string();
    println!("Day: {day}");

    // Not effective way before enum exist
    // Cost of maintenance, flexible, easy re-use
    let week_day = vec![
        "Monday".to_string(),
        "Tuesday".to_string(),
        "Wednesday".to_string(),
        "Thursday".to_string(),
        "Friday".to_string(),
        "Saturday".to_string(),
        "Sunday".to_string(),
    ];

    // Before enums exist
    day = week_day[5].clone();
    println!("Day: {day}");

    // Enums come into play
    let day = WeekDay::Saturday;
    println!("Day: {:?}", day);

    let travel = TravelType::Car;
    println!(
        "Allowance of participant: {}",
        travel.travel_allowance(60.0)
    );

    let travel = TravelType2::Car(60.0);
    println!(
        "Allowance of participant (improved version): {}",
        travel.travel_allowance_improved()
    );
}
