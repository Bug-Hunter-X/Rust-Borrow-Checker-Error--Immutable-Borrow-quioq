fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    // The following line will panic because vec is already borrowed
    vec.push(3);
}