fn main() {
    // s is the owner of this String
    let s = String::from("hello");
    println!("Value: {}", s);
} // s goes out of scope and is dropped here
