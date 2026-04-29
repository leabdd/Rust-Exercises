use std::io::{self, stdin, Write};

fn main() {
    let mut input = String::new();
    print!("Enter a number:");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read line");

    let n: i8 = input.trim().parse::<i8>().expect("Please enter a valid number");

    println!("n: {}, n+1: {}",n , n+1);
}
