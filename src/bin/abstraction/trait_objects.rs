// --------------------------------------------
//          Trait Objects
// --------------------------------------------

/*
    Using Dynamic Dispatch(Box<dyn ...>) known as trait objects
    instead of trait bound (Static Dispatch). 
    
    Box is a smart pointer to some heap allocated data 
    which is commonly used to trait objects, allow us to define
    a type which implements a trait without knowing or 
    having knowledge of what type is at the compile time.

    Essential advantage of trait objects is flexibility and easy maintain of code
    trade of performance at runtime.
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

trait Shape: Draw {
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

/*
    What happen behide the hood is the compiler will generates specific code as below
    for each concrete type used with generic function like this to eliminating runtime overhead.

    This process of generating the specific specialized versions of function for each type is refered
    to as Monomorphization or Static Dispatch.

    Static Dispatch is when  the compiler know which concrete methods to call at compile time.
*/
fn shape_properties_static<T>(object: &T) 
where 
    T: Shape
{
    object.area();
    object.perimeter();
}

// This is auto generated code version by compiler from shape_properties<T>
fn shape_properties_rect(object: &Rectangle) 
{
    object.area();
    object.perimeter();
}

// This is auto generated code version by compiler from shape_properties<T>
fn shape_properties_square(object: &Square) 
{
    object.area();
    object.perimeter();
}


/*
    In contrast to Statis Dispatch, We have another approach
    called Dynamic Dispatch where the specific implementation 
    will not be generated at compile time.

    "dyn" Stands for dynamics dispatch
*/
fn shape_properties_dynamic(object: Box<dyn Shape>) 
{
    object.area();
    object.perimeter();
}

/*
    The compiler gives an error because only one single type
    which implements the shape should be returned. Or we can say
    boolean x confuse compiler which specific concrte type function
    should be generated.
*/
// fn return_shape() -> impl Shape {
//     let sq = Square {
//         side: 5.0,
//         line_width: 5,
//         color: "Red".to_string(),
//     };

//     let rect = Rectangle {
//         length: 5.0,
//         width: 10.0,
//         line_width: 5,
//         color: "Red".to_string(),
//     };

//     let x = false;
//     if x {
//         sq
//     }else {
//         rect
//     }
// }

/*
    Modify function to return trait objects(Box) to fix the problem.

    The exact type of the return value in this case will be determined 
    at execution time and not compile time.
*/
fn return_shape() -> Box<dyn Shape> {
    let sq = Square {
        side: 5.0,
        line_width: 5,
        color: "Red".to_string(),
    };

    let rect = Rectangle {
        length: 5.0,
        width: 10.0,
        line_width: 5,
        color: "Red".to_string(),
    };

    let x = false;
    if x {
        Box::new(sq)
    }else {
        Box::new(rect)
    }
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

    // shape_properties_static(&r1);
    // shape_properties_static(&s1);

    /*
        This is generated code will replace code aobve 
        by compiler that really actual work on runtime .
    */
    shape_properties_rect(&r1);
    shape_properties_square(&s1);

    /*
        Dynamic Dispatch new function passing instance of box smart pointer.
        A new heap allocation where the value is being stored and returns
        a pointer to that value.
    */
    shape_properties_dynamic(Box::new(r1));
    shape_properties_dynamic(Box::new(s1));
}