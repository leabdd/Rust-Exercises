fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.retain(|&x| x % 2 != 0);

    println!("{:?}", vec)
}
