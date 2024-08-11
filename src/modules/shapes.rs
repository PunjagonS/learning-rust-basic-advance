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

    pub fn new_2(radius: f32) -> Circle {
        match radius {
            -10.0..= 0.0 => panic!("is between -10.0 and 0.0"),             // Arm 1 -10.0 ถึง 0.0
            ..= -10.0 => panic!("is less than -10.0"),                      // Arm 2 -n ถึง -10.0
            _ => Circle{ radius },
        }
    }

    pub fn contains(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }
}