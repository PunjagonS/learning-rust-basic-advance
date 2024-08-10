// --------------------------------------------
//          Unit Testing (Basics)
// --------------------------------------------

#[cfg(test)]
mod tests {
    use super::super::*;                                               // Import all parent modules

    #[test]
    fn larger_circle_should_contain_smaller(){
        let larger_circle = Circle::new(5.0);
        let smaller_circle = Circle::new(2.0);
        assert_eq!(larger_circle.contains(&smaller_circle), true);                                      // Macro equal
        // assert_eq!(larger_circle.contains(&smaller_circle), true, "Custom failure message");         // Add custom failure message

        assert_ne!(larger_circle.contains(&smaller_circle), false);                                     // Macro not equal
        assert!(larger_circle.contains(&smaller_circle));                                               // Macro default boolean "true"
    }

    #[test]
    fn smaller_circle_should_not_contain_larger(){
        let larger_circle = Circle::new(5.0);
        let smaller_circle = Circle::new(2.0);
        assert_eq!(!smaller_circle.contains(&larger_circle), true);
    }
}