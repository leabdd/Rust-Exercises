fn main() {
    // This is a String that can be changed and is in heap
    let mut str = String::from("Hello, world!");

    // This is like a char that cant be changed and is in a read only section
    let s: &str = "Hello, world!";

    println!("String: {}", str); 
    println!("&str:  {}", s);
}
