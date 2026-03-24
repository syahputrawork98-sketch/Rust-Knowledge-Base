fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // MOVE occurs (Heap data)

    // println!("{}", s1); // ERROR: borrow of moved value: `s1`
    println!("s2: {}", s2);

    // Deep copy using clone
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);
}
