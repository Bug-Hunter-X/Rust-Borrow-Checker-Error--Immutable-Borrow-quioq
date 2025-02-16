fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    // To fix, create a new vector or clone the vector and change it.
    //Here's an example of creating a new vector to store the new element without changing the original vector
    let mut new_vec = vec.clone();
    new_vec.push(3);
    println!("New vector: {:?}", new_vec);
    println!("Original vector: {:?}", vec);
}    
