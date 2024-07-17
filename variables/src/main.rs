fn main() {
    // we have to provide a type for the constants
    const CONSTANT: u32 = 100;
    let x = 1;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}