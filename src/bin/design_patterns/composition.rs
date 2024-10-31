// --------------------------------------------
//            Composition Pattern
// --------------------------------------------

/*
    Example of composition pattern using enum instead of trait
    for more complex structs make code more cleaner and faster,
    avoid using Box<dyn Trait> moving workload to runtime.
*/

////////////////////////// Trait /////////////////////////
//////////////////////////////////////////////////////////
// trait Animal {
//     fn speak(&self);
// }

struct Dog {
    name: String,
    age: u8,
}
struct Cat {
    color: String,
    gender: char,
}

// impl Animal for Dog {
//     fn speak(&self) {
//         println!("Bark");
//     }
// }

// impl Animal for Cat {
//     fn speak(&self) {
//         println!("Meow");
//     }
// }
//////////////////////////////////////////////////////////

////////////////////////// Enum //////////////////////////
enum Animal {
    Dog(Dog),
    Cat(Cat),
}

trait Sound {
    fn speak(&self);
}

impl Sound for Dog {
    fn speak(&self) {
        println!("Bark");
    }
}

impl Sound for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}
//////////////////////////////////////////////////////////

// Attempt to create more complex structs by composing other animals
struct Zoo {
    // animals: Vec<Box<dyn Animal>>, // Bad practice
    animals: Vec<Animal>, // Improved
}

impl Zoo {
    fn new() -> Self {
        Zoo {
            animals: Vec::new(),
        }
    }

    // fn add_animal(&mut self, animal: Box<dyn Animal>) {  // Bad practice
    fn add_animal(&mut self, animal: Animal) {
        self.animals.push(animal);
    }

    fn make_sounds(&self) {
        for animal in &self.animals {
            match animal {
                Animal::Dog(dog) => dog.speak(),
                Animal::Cat(cat) => cat.speak(),
            }
        }
    }
}

fn main() {
    let dog = Dog {
        name: "Rex".to_string(),
        age: 5,
    };

    let cat = Cat {
        color: "Black".to_string(),
        gender: 'M',
    };

    let mut zoo = Zoo::new();
    // zoo.add_animal(Box::new(dog));
    // zoo.add_animal(Box::new(cat));

    zoo.add_animal(Animal::Dog(dog));
    zoo.add_animal(Animal::Cat(cat));
    zoo.make_sounds();
}
