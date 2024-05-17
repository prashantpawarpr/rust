struct Person {
    name: String,
    age: u32,
    place: String,
}
fn main() {
    let name = String::from("John");
    let age = 12;
    let place = String::from("san-fransico");
    let john = Person { name, age, place };
    println!("{:?}", john);
}

