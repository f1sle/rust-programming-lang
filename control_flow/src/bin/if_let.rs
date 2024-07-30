fn main() {
    let condition = true;
    let number = if !condition { 5 } else { 10_000 };
    println!("The number is: {number}"); // 5
  }