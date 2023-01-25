pub fn run() {
    let age = 18;
    let check_id = false;

    //if else
    if age >= 21 && check_id{
        println!("ok to enter" )
    } else if age < 21 && check_id{
        println!("no drink")
    } else {
        println!("need id")
    }
}