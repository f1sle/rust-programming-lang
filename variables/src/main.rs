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
    // addition
    let sum = 5 + 10;
    println!("Sum is: {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("Diff is: {difference}");
    // multiplication
    let product = 4 * 30;
    println!("Prod is: {product}");
    // division
    let quotient = 56.7 / 32.2;
    println!("Quot is: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("Trunk is: {truncated}");
    // remainder
    let remainder = 43 % 5; 
    println!("Rem is: {remainder}");
}