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


/////////////////////////////////////////////////////////////////////////
/*
    Use Case 2: represent different design choices for modeling a Person 
        with potentially different types of addresses. While they both serve the purpose 
        of associating a person with an address, they are not directly interchangeable 
        because they represent different conceptual models. Here’s how they compare and differ:

    struct HomeAddress {
        street: String,
        city: String,
        state: String,
        zip_code: String,
    }

    struct WorkAddress {
        company_name: String,
        street: String,
        city: String,
        state: String,
        zip_code: String,
    }

    struct Person {
        name: String,
        home_address: Option<HomeAddress>,
        work_address: Option<WorkAddress>,
    }

    enum Address {
        Home(HomeAddress),
        Work(WorkAddress),
    }

    struct PersonEnum {
        name: String,
        address: Address,
    }

    Comparison of the Two Approaches
    1. Conceptual Model:
        - Option-based Structs: This model assumes that a person can have both a home address 
            and a work address, or either one, or none. Each type of address is optional (Option<HomeAddress> and Option<WorkAddress>).
        - Enum-based Struct: This model assumes that a person can have only one type of address 
            at a time: either a home address or a work address, but not both simultaneously. The Address enum captures this choice.

    2.Data Representation:
        - Option-based Structs: You can represent multiple addresses independently. 
            For instance, Person might have both a home and a work address, or just one of them.
        - Enum-based Struct: The Person struct can hold only one Address at a time, 
            ensuring mutual exclusivity between the address types.

    3. Data Usage:
        - Option-based Structs: Accessing and checking whether a Person has a home or work address involves checking each Option individually.

        if let Some(home_address) = &person.home_address {
            println!("Home Address: {}", home_address.street);
        }
        if let Some(work_address) = &person.work_address {
            println!("Work Address: {}", work_address.company_name);
        }

        - Enum-based Struct: Accessing the address requires pattern matching to determine which variant (home or work) is being used.

    4. Flexibility:
        - Option-based Structs: More flexible if you need to allow multiple addresses or need to potentially 
            add more types of addresses in the future. It also supports scenarios where a person might not have any address.
        - Enum-based Struct: Less flexible in terms of adding multiple address types but enforces a stricter model 
            where only one type of address can be associated with a person.

    5. Memory Usage:
        - Option-based Structs: This approach may use more memory because you have two Option fields, 
            which each carry overhead for indicating presence or absence.
        - Enum-based Struct: Potentially more memory-efficient as it only holds one address type 
            at a time and carries a small overhead for the enum tag.

    Replacing One with the Each other
        - Replacing Option-based Structs with Enum: You would lose the ability to store multiple addresses 
            for a person simultaneously. The model becomes simpler in terms of ensuring mutual exclusivity 
            but might not fit scenarios where multiple addresses are valid.
        - Replacing Enum with Option-based Structs: You gain the ability to store multiple addresses 
            but introduce more complexity in handling the possibility of either or both addresses being None.

    Conclusion
        These two designs are not directly interchangeable without altering the conceptual model of your data. 
        If your application requires a person to have multiple addresses, or potentially none, the Option-based approach is better. 
        If a person should have only one address type at a time, the enum-based approach is more appropriate. 
        Your choice should depend on the specific requirements of your application.
*/


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
