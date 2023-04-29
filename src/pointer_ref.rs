// Reference Pointers - Point to a resource in memory

use std::vec;

pub fn run() {
    // Primitive Array
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 3] = arr1;

    println!("{:?}", (arr1, arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource

    // Vector
    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2: &Vec<i32> = &vec1;

    println!("{:?}", (&vec1, vec2));
}
