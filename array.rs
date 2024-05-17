use std::mem;
fn main() {
    let arr: [i32; 5] = [0, 1, 2, 3, 4];

    let _arr1: [i32; 50] = [0; 50];
    //we can initilize same value multiple times
    //first parameter the value we want to inserat
    //second ammount of time we want to insert
    println!("{:?}", arr);
    // println!("{:?}", arr1);
    println!("{}", arr.len());
    println!("{}", mem::size_of_val(&arr));
    //gives the memory  of the array have use method called size_of_val
    //with the pointer of array;
}
