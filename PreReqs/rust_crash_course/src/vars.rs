//crtl j opens the terminal
//ctrl b opens the side bar
//ctrl shift e open the explorer bar

//variables hold primitive data or reference to data
//variables are immutable by default
//Rust is a block scoped language

pub fn run () {
    let name = "Brad";

    let mut age = 32;
    age = 33;

    println!("My name is {} and I am {}", name, age);

    //define constat
    //consts typical are UPPERCASE
    const ID: i32 = 001;
    println!("ID: {}", ID);
}