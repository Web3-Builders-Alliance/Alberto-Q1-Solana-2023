pub fn run() {
    //while loop fizz buzz
    let mut count = 0;
    // while count <= 50 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if count % 3 == 0{
    //         println!("fizz");
    //     } else if count % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", count);
    //     }
    //     count += 1;
    // }

    //for range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
            //crtl d to select the same word occurence down
        } else if x % 3 == 0{
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}