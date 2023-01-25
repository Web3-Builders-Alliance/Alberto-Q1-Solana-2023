//structs - used to create custom data types

//tradtional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

//tuple struct
//struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name:String
}

//then you implement values to the struct
//and you can put fn in it
impl Person {
    //construct person
    //the arrow means the fn is going to return -> a Person
    fn new (first: &str, last: &str) -> Person {
        Person {
            //because you use String in the struct above, the element have to be to_string
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    //Get full name
    //you are using self similar to this in other lng, self reference
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name or modify
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //Name as tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     //call the values from struct and add the values
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };

    // //to change values of the struct
    // c.red = 199;

    // //access the colors with the dot syntax
    // println!("Traditional Color: {} {} {}", c.red, c.green, c.blue);

    //calling the tuple struct
    // let mut c = Color(255, 0 , 0);

    // //you can also change values
    // c.0 = 199;

    // //access with dot syntax and index
    // println!("Tuple Color: {} {} {}", c.0, c.1, c.2);

    //You are using ::new since new is the name of the fn above
    let mut p = Person::new("John", "Smith");
    //println!("Person {} {}", p.first_name, p.last_name);

    //Using the full_name method you only need one placeholder
    //So this is a straingt calling, rather than using the ::new format
    println!("Person {}", p.full_name());

    //using the set_last_name
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());

    //name as tuple
    println!("Person as Tuple{:?}", p.to_tuple());
}