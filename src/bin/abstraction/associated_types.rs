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

    * We will use the Associated Types when there is only a single implement
    of the trait per type. If there are multiple implementation of the same type
    for a given trait, then we will use generics. *
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

//// Code duplicate & redundancy
// impl Kmh {
//     fn distance_in_three_hours(&self) -> Km {
//         Km {
//             value: self.value * 3,
//         }
//     }
// }

//// Code duplicate & redundancy
// impl Mph {
//     fn distance_in_three_hours(&self) -> Miles {
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }

////////////////////////////////////////////////////////////////////////////////////////////////

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

/*
    Example apply Generics with Associate Types 
    to allow us implement multiple times.
*/

trait Addition<T, U> {
    fn add(self, rhs: T) -> U;
}

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

// First implement trait
impl Addition<Point, Point> for Point {
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Second implement trait
impl Addition<i32, Point> for Point {
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

// Third implement trait
impl Addition<Point, Line> for Point {
    fn add(self, rhs: Point) -> Line {
        Line {
            start: self,
            end: rhs,
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


    /////////////////////////////////////////////////////////////////////////////////
    let p1 = Point { x:1, y:1 };
    let p2 = Point { x:2, y:2 };

    /*
        Need to giving an explicit type for p3 because self.add(Point) is matched 2 implements 
        at 1 and 3 as 1 return Point and 3 return line so possible output for p3 is Point or Line
    */
    let p3: Point = p1.add(p2);

    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x:1, y:1 };
    let p3 = p1.add(2);

    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x:1, y:1 };
    let p2 = Point { x:2, y:2 };
    let line: Line = p1.add(p2);

    assert!(line.start.x == 1 && line.start.y == 1 && line.end.x == 2 && line.end.y == 2 );
}