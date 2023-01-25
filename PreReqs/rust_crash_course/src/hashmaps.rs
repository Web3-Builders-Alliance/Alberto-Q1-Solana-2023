use std::collections::HashMap;

//Hash maps are objects that allow to store data in a key:value pair


pub fn run() {
    let mut map = HashMap::new();

    map.insert(0, "H1");
    map.insert(1, "Hi2");
    println!("{:?}", map);

    //the get reutns an Option not a value
    match map.get(&0) {
        Some(str) => println!("{}", str),
        None => println!("Doesn't exist in map"),
    }

    match map.get(&2) {
        Some(str) => println!("{}", str),
        None => println!("Doesn't exist in map"),
    }

    map.remove(&0);
    println!("{:?}", map);

}