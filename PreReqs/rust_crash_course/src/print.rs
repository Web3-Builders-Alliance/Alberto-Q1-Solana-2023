//ctrl j for switching between terminal and code
pub fn run() {
    //print to console
    println!("Hello from the print rs file");

    //Positional Args
    //For when you need to use a value more than once
    println!("{0} is from {1} and {0} likes {2}", "Alberto", "Mexico", "tacos");

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
}