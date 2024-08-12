// --------------------------------------------
//          Generics
// --------------------------------------------

/*
    Generics do not incur any runtime performance cost 
    due to feature called Monomorphization.
*/

// Before apply generic
struct Point {
    x: i32,
    y: i32,
}

struct Point2<T> {
    x: T,
    y: T,
}

struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point3<T, U> {
    fn new(x: T, y: U) -> Point3<T, U> {
        Point3 { x, y }
    }
}

impl Point3<i32, i32> {         // Using concrete type of i32 example
    fn printing(&self) {
        println!("The value of the coordinates are {}, {}", self.x, self.y);
    }

    /* 
        Error from compiler has on way to decide 
        which specific function should be invoke
        between concrete and generic
    */
    // fn new(x: i32, y: i32) -> Point3<i32, i32> {
    //     Point3 { x, y }
    // }

    fn new_1(x: i32, y: i32) -> Point3<i32, i32> {
        Point3 { x, y }
    }
}

impl Point3<f64, f64> {
    fn printing(&self) {
        println!("The value of the coordinates are {}, {}", self.x, self.y);
    }
}

fn add_points<T, U>(p1: &Point3<T, U>, p2: &Point3<T, U>) -> Point3<T, U> {
    unimplemented!();                                      // Marco for tell compiler we will implement it later so not show error
}

fn add_points_i32(p1: &Point3<i32, i32>, p2: &Point3<i32, i32>) -> Point3<i32, i32> {
    unimplemented!();                                      // Marco for tell compiler we will implement it later so not show error
}

fn add_points_i64(p1: &Point3<i64, i64>, p2: &Point3<i64, i64>) -> Point3<i64, i64> {
    unimplemented!();                                      // Marco for tell compiler we will implement it later so not show error
}


fn main() {
    let p = Point { x: 0, y: 0 };
    //let p2 = Point { x: 1.0, y: 4.0 };                    // Eror mismatched types before apply generic

    let p2 = Point2 { x: 1.0, y: 4.0 };
    // let p3 = Point2 { x: 1, y: 4.0 };

    // let p3 = Point3 { x: 1, y: 4.0 };
    let p3 = Point3::new( 1, 4.0 );
    // p3.printing();                                        // Error from impl generic but fn printing is support only concrete type

    let p4 = Point3::new( 0, 0 );
    let p5 = Point3::new( 1.0, 4.0 );

    add_points(&p4, &p4);
    add_points(&p5, &p5);

}