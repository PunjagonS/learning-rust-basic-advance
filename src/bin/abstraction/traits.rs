// --------------------------------------------
//          Traits
// --------------------------------------------

/*
    Traits is the rust way to share functionality and provide a common interface 
    like Inheritance && Polymorphism from OOP but ONLY functionality can be share. 
*/

struct drawing_info {
    line_width: u8,
    color: String,
}

struct Square {
    side: f32,
    // line_width: u8,
    // color: String,
    info: drawing_info,
}

struct Rectangle {
    length: f32,
    width: f32,
    // line_width: u8,
    // color: String,
    info: drawing_info,
}

/* 
    Before traits come into play. we develop with concerned 
    difference implement of calculation area within Square and Rectangle
*/
// impl Square {
//     fn calculate_area(&self) {
//         println!("The area is: {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.length * self.width
//     }
// }

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

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

    // Use default implementation of Shape instead
    // fn perimeter(&self) -> f32 {}
}

fn main() {
    let r1 = Rectangle {
        width: 5.0,
        length: 4.0,
        // line_width: 1,
        // color: "Red".to_string(),
        info: drawing_info{ line_width: 1, color: "Red".to_string() }
    };

    let s1 = Square {
        side: 3.2,
        // line_width: 1,
        // color: "Red".to_string(),
        info: drawing_info{ line_width: 1, color: "Red".to_string() }
    };

    r1.area();
    s1.area();

    r1.perimeter();
    s1.perimeter();
}