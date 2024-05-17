fn main() {
    let a = (12, 32);
    println!("{}", a.0);

    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));
    let tuple = (10u32, 23.32, 2u8);
    let (x, y, z) = tuple;
    println!("{},{},{}", x, y, z);
    println!("{:?}", tuple);
}
