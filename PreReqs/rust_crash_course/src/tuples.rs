//tuples group together values of different types
//max 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("bard", "georgia", 32);

    println!("{} is from {} and age", person.0, person.1);

}

//Another example
fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age)/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}