// #[derive(Debug)]
#[derive(Debug, Default)]
pub struct Student {
    pub id: u8,
    pub age: u8,
    pub name: String,
}

impl Student {
    pub fn new(std_name: String) -> Result<Self, String> {
        if std_name.chars().all(|x| matches!(x, 'a'..= 'z')) {
            Ok(Self {
                id: 0,
                age: 20,
                name: std_name,
            })
        } else {
            Err("The name is invalid".to_string())
        }
    }
}

// Comment to use derive macro 'Default' instead
// impl Default for Student {
//     fn default() -> Self {
//         Self {
//             id: 0,
//             age: 20,
//             name: "Unknown".to_string(),
//         }
//     }
// }