fn main() {
    let arr: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    //let mut idx = 0;

    // working but error prone approach
    //while idx < arr.len() {
    //    println!("The value is: {}", arr[idx]);
    //    idx += 1;
    //}
    //
    // here is a better alternative with for
    for element in arr {
        println!("The value is: {element}");
    }
}