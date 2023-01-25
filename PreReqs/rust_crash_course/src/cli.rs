use std::env;

pub fn run() {
    //by doing this, you will take input from the
    // cargo run line and whaeverr is after will be a str

    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Alberto";
    let status = "100%";

    //password protected CLI
    let password = args[2].clone();

    //println!("Command: {}", command);
    if command == "hello" && password == "anime"{
        println!("Hi {}, this is your CLI", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Unathorized User!");
    }
}