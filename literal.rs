// fn reverse(pair: (32, bool)) -> (bool, i32) {
//     let (int_param, bool_param) = pair;
//     return (bool_param, int_param);
// }
fn main() {
    // println!("1+2 =  {}", 1i32 - 2);
    // println!("1e4 {} and {}", 1e4, -23e-4);
    // let long_tuple = (12i32, 11i32, 10u32, 1u32);
    // println!("the first value of tuple is {} ", long_tuple.0);
    // println!("the first value of tuple is {} ", long_tuple.1);
    // println!("{:?}", reverse((12, true)));
    // tuple can contain tuples
    let inside_tuple = ((12i16, 10i16), (12i16, 10i16), 10i16);
    println!("{:?}", inside_tuple.0 .0);
    //long tuples
    // let long_tuple = (
    //     1u8, 2u8, 3u8, -1i16, -2i16, 1u8, 2u8, 3u8, -1i16, -2i16, 1u8, 2u8, 3u8, -1i16, -2i16,
    // );
    // println!("{}", long_tuple);
}
