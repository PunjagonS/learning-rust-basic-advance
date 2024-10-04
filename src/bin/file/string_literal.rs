// --------------------------------------------
//               String Literal
// --------------------------------------------

fn main() {
    let str = "The main said \" Hello world \"";
    println!("{}", str);

    let str = r#"The main said " Hello world ""#;
    println!("{}", str);

    let str = r"The main said _Hello world_ \n \t ' ";
    println!("{}", str);

    let jason_str = "{
        \"name\": \"John\",
        \"age\": 20,
        \"sex\": \"Male\"
    }";
    println!("{}", jason_str);

    let jason_str = r#"{
        "name": "Micheal",
        "age": 20,
        "sex": "Female"
    }"#;
    println!("{}", jason_str);
    let str = r##"Hello"# World!"##; // Distinguish between # used as string and # used as delimiter of string.
    println!("{}", str);
}
