fn main() {
    let arr: [i32; 5] = [0, 1, 2, 3, 4];

    let arr1: [i32; 500] = [0; 500];
    //we can initilize same value multiple times
    //first parameter the value we want to inserat
    //second ammount of time we want to insert
    println!("{:?}", arr);
    println!("{:?}", arr1);
}
