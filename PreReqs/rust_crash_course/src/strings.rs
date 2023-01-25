//crtl j to open terminal
//crt to open and hide sidebar
//crtl shift e to focus on explorer side bar

//primitive string = immutable fixed lenght string somehwrre in memory
//string = growable, heap-allocate data structure - use when you need to modify or own
//              string data

pub fn run(){
    //you need the String::from in order to make the var a
    //string that can be added or reduce
    let mut hello = String::from("hello ");

    //get len
    println!("Length: {}", hello.len());

    //push char
    hello.push('w');

    //push str
    hello.push_str("orld!");

    //capacity in bytes
    println!("capacity: {}", hello.capacity());

    //check if empty
    println!("Is empty: {}", hello.is_empty());

    //contains - case sensitive
    println!("Contains 'World': {}", hello.contains("world"));

    //replace
    println!("Replace: {}", hello.replace("world", "there"));

    //loop through string by whitespace
    //what this will do is break the string into multiple lines if a whitespace is found
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    //create str with capacity and later check assertion
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion testing
    //if at anytime the value on the left chanes, then code will error as it says doesnt match

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);


    //println!("{}", hello);
}