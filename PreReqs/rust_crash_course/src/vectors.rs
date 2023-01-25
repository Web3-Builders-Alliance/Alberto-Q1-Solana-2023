//unlike arrays, you can add and sub items in a vector
pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //get single value
    println!("Single value: {}", numbers[0]);

    //adding numbers to the vector
    numbers.push(6);
    numbers.push(8);
    numbers.push(10);

    //removing last item
    numbers.pop();

    //re-assign value
    numbers[2] = 12;
    println!("{}", numbers[2]);

    println!("{:?}", numbers);

    //looping through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //loop and mut vectors
    for x in numbers.iter_mut(){
        *x *=2;
    }
    println!("{:?}", numbers);


}