// --------------------------------------------
//              Builder Pattern
// --------------------------------------------

#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: MembershipType,
    gender: char,
    country: String,
    age: u8,
}

impl Customer {
    // fn new(name: String) -> Self {
    //     Customer {
    //         name: name,
    //         ..Default::default()
    //     }
    // }

    // fn new_2(name: String, username: String) -> Self {
    //     Customer {
    //         name: name,
    //         username: username,
    //         ..Default::default()
    //     }
    // }

    // fn new_3(name: String, username: String, membership: MembershipType) -> Self {
    //     Customer {
    //         name: name,
    //         username: username,
    //         membership: membership,
    //         ..Default::default()
    //     }
    // }

    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name: name,
            // username: None,
            // membership: None,
            // gender: None,
            // country: None,
            // age: None,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
enum MembershipType {
    New,
    Casual,
    Loyal,
}

impl Default for MembershipType {
    fn default() -> Self {
        MembershipType::New
    }
}

// #[derive(Default)]
// struct CustomerBuilder {
//     name: String,
//     username: Option<String>,
//     membership: Option<MembershipType>,
//     gender: Option<char>,
//     country: Option<String>,
//     age: Option<u8>,
// }

// impl CustomerBuilder {
//     fn username(&mut self, username: String) -> &mut Self {
//         self.username = Some(username);
//         self
//     }

//     fn membership(&mut self, membership: MembershipType) -> &mut Self {
//         self.membership = Some(membership);
//         self
//     }

//     fn gender(&mut self, gender: char) -> &mut Self {
//         self.gender = Some(gender);
//         self
//     }

//     fn country(&mut self, country: String) -> &mut Self {
//         self.country = Some(country);
//         self
//     }

//     fn age(&mut self, age: u8) -> &mut Self {
//         self.age = Some(age);
//         self
//     }

//     fn build(&mut self) -> Customer {
//         Customer {
//             name: std::mem::take(&mut self.name),
//             username: self.username.take().unwrap_or_default(),
//             membership: self.membership.take().unwrap_or_default(),
//             gender: self.gender.take().unwrap_or_default(),
//             country: self.country.take().unwrap_or_default(),
//             age: self.age.take().unwrap_or_default(),
//         }
//     }
// }

macro_rules! create_builder {
    (
        $(#[$derive_meta:meta])*
        $builder_name:ident,
        $struct_name:ident {
            required: { $($req_field:ident: $req_type:ty),* $(,)? },
            optional: { $($opt_field:ident: $opt_type:ty),* $(,)? }
        }
    ) => {
        $(#[$derive_meta])*
        struct $builder_name {
            $( $req_field: $req_type, )*
            $( $opt_field: Option<$opt_type>, )*
        }

        impl $struct_name {
            fn builder($($req_field: $req_type),*) -> $builder_name {
                $builder_name {
                    $( $req_field, )*
                    $( $opt_field: None, )*
                }
            }
        }

        impl $builder_name {
            $(
                fn $opt_field(&mut self, value: $opt_type) -> &mut Self {
                    self.$opt_field = Some(value);
                    self
                }
            )*

            fn build(&mut self) -> $struct_name {
                $struct_name {
                    $( $req_field: std::mem::take(&mut self.$req_field), )*
                    $( $opt_field: self.$opt_field.take().unwrap_or_default(), )*
                }
            }
        }
    }
}

// Call macro to create CustomerBuilder
create_builder!(
    #[derive(Default)]
    CustomerBuilder,
    Customer {
        required: { name: String },
        optional: {
            username: String,
            membership: MembershipType,
            gender: char,
            country: String,
            age: u8,
        }
    }
);

fn main() {
    // let new_usre = Customer::new("Nouman".to_string());
    // let user_with_login = Customer::new_2("Joseph".to_string(),
    //     "joe1234".to_string());
    // let user_with_membership = Customer::new_3(
    //     "Micheal".to_string(),
    //     "micheal2000".to_string(),
    //     MembershipType::loyal,
    // );

    // Using Builder original
    // let new_user = Customer::new("Nouman".to_string()).build();
    // let user_with_login = Customer::new("Joseph".to_string())
    //     .username("joe123".to_string())
    //     .build();

    // let user_with_membership = Customer::new("Micheal".to_string())
    //     .username("micheal2000".to_string())
    //     .membership(MembershipType::Loyal)
    //     .build();

    //////////////////////////////////////////////////////////////////////////////////////////////
    // Using Builder created by macro_rule!
    let new_user = Customer::builder("Nouman".to_string()).build();
    let user_with_login = Customer::builder("Joseph".to_string())
        .username("joe123".to_string())
        .build();

    let user_with_membership = Customer::builder("Micheal".to_string())
        .username("micheal2000".to_string())
        .membership(MembershipType::Loyal)
        .build();
    //////////////////////////////////////////////////////////////////////////////////////////////

    println!("{:?}", new_user);
    println!("{:?}", user_with_login);
    println!("{:?}", user_with_membership);
}
