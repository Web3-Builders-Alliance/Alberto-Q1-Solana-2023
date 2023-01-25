pub fn run() {
    //primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("values {:?}", (arr1, arr2));


    //with non primitives, if you assign another var to a piece of data, the first
    //var will no longer hold that value. You'll need to use a reference (&) to point to the
    //resource

    //vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));

}