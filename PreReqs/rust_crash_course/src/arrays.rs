//arrays - fixed list where elements are the same data types

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //get single value
    println!("Single value: {}", numbers[0]);

    //re-assign value
    numbers[2] = 12;
    println!("{}", numbers[2]);


}