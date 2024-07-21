// --------------------------------------------
//          Enums
// --------------------------------------------

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
