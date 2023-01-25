pub fn run() {
    greeting("hello","Jone");

    //bind fn values to var
    let get_sum = add(8, 6);
    println!("{}", get_sum);

    //closure fn method, you can use outside var
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    //the n1 n2 are coming from the parms passed in the fn below
    println!("C Sum {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

//function to return value
fn add(n1: i32, n2: i32) -> i32{
    n1 + n2
}