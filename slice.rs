fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}
fn main() {
    //array can be automatically borrowed as slices
    let arr = [1, 2, 3, 4, 5, 6];
    let arr1: [i32; 500] = [1; 500];
    analyze_slice(&arr);
    println!("{}", arr[0]);
    println!("{}", arr.len());
    println!("{}", arr1.len());
    //slice can point to the specific section of the array
    //they are of form [starting_index..ending_index]
    //starting index is the starting index of the slice
    //ending_index is the ending index of the slice
    analyze_slice(&arr[0..5]);
    println!("{:?}", &arr[0..5]);
    //creating empty slices
    let empty_arr: [u32; 0] = [];
    println!("{:?}", empty_arr);
    //assert assert_eq! is a macro used for testing equality between two values.
    //It's commonly used in unit tests to ensure that the actual result
    //of a function or expression matches the expected result.
    println!("{:?}", assert_eq!(&empty_arr, &[]));
    println!("{:?}", assert_eq!(&empty_arr, &[][..]));
    //same but more verbose;
    //array can be safely access using '.get', which returns an 'Option'.
    //This can be matched as shown below or used with the expect()
    //if you want to exit the program with nice message instead of
    //happily continue

    for i in 0..arr.len() + 1 {
        match arr.get(i) {
            Some(xval) => println!("{}:{}", i, xval),
            None => println!("slow down {} is far away ", i),
        }
    }
}
