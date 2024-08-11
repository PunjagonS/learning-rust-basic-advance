// --------------------------------------------
//          Unit Testing (Basics)
// --------------------------------------------

#[cfg(test)]
mod tests {
    use super::super::*;                                                                         // Import all parent modules

    #[test]
    fn larger_circle_should_contain_smaller(){
        let larger_circle = Circle::new(5.0);
        let smaller_circle = Circle::new(2.0);
        // assert_eq!(larger_circle.contains(&smaller_circle), true);                             // Macro equal
        assert_eq!(larger_circle.contains(&smaller_circle), true, "Custom failure message");      // Add custom failure message

        assert_ne!(larger_circle.contains(&smaller_circle), false);                               // Macro not equal
        assert!(larger_circle.contains(&smaller_circle));                                         // Macro default boolean "true"
    }

    #[test]
    fn smaller_circle_should_not_contain_larger(){
        let larger_circle = Circle::new(5.0);
        let smaller_circle = Circle::new(2.0);
        assert_eq!(!smaller_circle.contains(&larger_circle), true);
    }

    /*  Example of using standard lib Result 
        to define this test case failed or pass
        instead of used assert directly.
     */
    #[test]
    fn should_not_create_circle() -> Result<(), String> {
    Circle::new_1(0.01)?;                                                               // "?" shorthand to return Err if new_1() is error
        // Circle::new_1(-0.1)?;                                                                                                            
        Ok(())
    }

    #[test]
    #[should_panic(expected = "is less than -10.0")]                                            // Expect panic with exact error string 
    // #[should_panic]
    fn should_not_creat_and_panic() {
        Circle::new_2(-11.0);
    }
}