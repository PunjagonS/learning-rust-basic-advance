pub struct Circle {
    radius: f32,
}

impl Circle{
    pub fn new(radius: f32) -> Circle {
        Circle{radius}
    }

    pub fn new_1(radius: f32) -> Result<Circle, String> {
        if radius >= 0.0 {
            Ok(Circle {radius})
        }else {
            Err("radius should be positive.".to_string())
        }
    }

    pub fn contains(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }
}