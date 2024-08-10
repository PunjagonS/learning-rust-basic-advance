pub struct Circle {
    radius: f32,
}

impl Circle{
    pub fn new(radius: f32) -> Circle {
        Circle{radius}
    }

    pub fn contains(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }
}