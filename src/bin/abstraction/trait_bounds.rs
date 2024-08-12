// --------------------------------------------
//          Trait Bounds
// --------------------------------------------

/*
    Trait Bounds is to specify generic type by having colon(:)
    and then the name of trait.
*/
struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

trait Shape2 {}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("Rectangle area: {}", area_of_rect);
        area_of_rect
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("Rectangle Perimeter : {}", perimeter_of_rect);
        perimeter_of_rect
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }
}

// Syntax 1
fn ShapeProperties1<T: Shape>(object: T) {}

// Syntax 2 -> for trait bounds but work the same.
fn ShapeProperties2<T>(object: impl Shape) {}

// Syntax 3 -> use plus(+) symbol for multiple trait bounds.
fn ShapeProperties3<T: Shape + Shape2>(object: T) {}

// Syntax 4(Most popular) -> use "where" for trait bounds. valid both single and multiple or more complex.
fn ShapeProperties4<T, U>(object: T, object2: U) 
where 
    T: Shape + Shape2, 
    U: Shape2, 
{}

fn ShapeProperties<T>(object: T) where T: Shape,
{
    object.area();
    object.perimeter();
}

// This fn can return anything that implement from Shape
fn ReturnShape() -> impl Shape {
    let sq = Square {
        side: 5.0,
        line_width: 5,
        color: "Red".to_string(),
    };
    sq

    // let rect = Rectangle {
    //     length: 5.0,
    //     width: 10.0,
    //     line_width: 5,
    //     color: "Red".to_string(),
    // };

    // let x = false;
    // if x {
    //     sq
    // }else {
    //     rect
    // }
}

struct Circle {
    radius: f32,
}

fn main() {
    let r1 = Rectangle {
        width: 5.0,
        length: 4.0,
        line_width: 1,
        color: "Red".to_string(),
    };

    let s1 = Square {
        side: 3.2,
        line_width: 1,
        color: "Red".to_string(),
    };

    let c1 = Circle {
        radius: 5.0
    };

    ShapeProperties(r1);
    ShapeProperties(s1);
    // ShapeProperties(c1);             // Error cause from Circle not bound to Shape
}