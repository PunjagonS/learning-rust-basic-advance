// --------------------------------------------
//          Super Traits
// --------------------------------------------

/*
    Super Traits is the concept closely related to inheritance.

    The traits to which we extend a certain trait is known as a super trait.
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

trait Draw {
    fn draw_object(&self);
}

// Trait Draw is the super trait of trait Shape
trait Shape: Draw + OtherTrait + SomeOtherTrait {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

trait OtherTrait {}                         // No items trait like this is known as Marker Traits.
impl OtherTrait for Rectangle {}
impl OtherTrait for Square {}

trait SomeOtherTrait {}                     // No items trait like this is known as Marker Traits.
impl SomeOtherTrait for Rectangle {}
impl SomeOtherTrait for Square {}

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

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Draw Rectangle");
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("Draw Square");
    }
}

fn shape_properties<T>(object: T) 
where 
    //T: Shape + OtherTrait + SomeOtherTrait,                         // Reduce this long list trait bounds by super traits
    T: Shape
{
    object.area();
    object.perimeter();
}

// This fn can return anything that implement from Shape
fn return_shape() -> impl Shape {
    let sq = Square {
        side: 5.0,
        line_width: 5,
        color: "Red".to_string(),
    };
    sq
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

    shape_properties(r1);
    shape_properties(s1);
    // shape_properties(c1);             // Error cause from Circle not bound to Shape
}