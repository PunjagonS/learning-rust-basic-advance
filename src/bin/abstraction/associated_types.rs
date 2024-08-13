// --------------------------------------------
//          Associated Types in Traits
// --------------------------------------------

/* 
    Associated Types in traits allow us to define placeholder types
    within a trait, where the concrete type is determined by implementing type
    providing flexibility for trait implementer to choose the specific types, 
    which makes sense for their particular implementation.

    Associated Types within trait are declared using the syntax of type,
    followed by the name of the associated type.
*/

#[derive(Debug)]
struct Km {
    value: u32,
}

#[derive(Debug)]
struct Kmh {
    value: u32,
}

#[derive(Debug)]
struct Miles {
    value: u32,
}

#[derive(Debug)]
struct Mph {
    value: u32,
}

// impl Kmh {
//     fn distance_in_three_hours(&self) -> Km {
//         Km {
//             value: self.value * 3,
//         }
//     }
// }

// impl Mph {
//     fn distance_in_three_hours(&self) -> Miles {
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }

// Using Associated Types
trait DistanceThreeHours{
    type Distance;                                              // Define associated type.
    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    type Distance  = Km;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHours for Mph {
    type Distance  = Miles;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

fn main() {
    let speed_kmh = Kmh{ value: 90 };
    let distance_km = speed_kmh.distance_in_three_hours();

    println!("At {:?}, you will travel {:?} in 3 hours.", speed_kmh, distance_km);

    let speed_mph = Kmh{ value: 90 };
    let distance_miles = speed_mph.distance_in_three_hours();

    println!("At {:?}, you will travel {:?} in 3 hours.", speed_mph, distance_miles);
}