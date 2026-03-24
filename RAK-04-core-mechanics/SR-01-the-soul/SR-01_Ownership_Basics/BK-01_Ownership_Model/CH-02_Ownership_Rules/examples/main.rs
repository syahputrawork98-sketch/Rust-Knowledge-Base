fn main() {
    let s1 = String::from("hello"); // Rule 1: s1 is owner
    let _s2 = s1; // Rule 2: s1 is moved to s2 (only one owner)
    
    // println!("{}", s1); // Rule 2: This would error because s1 moved
    println!("Success: Ownership moved.");
} // Rule 3: s2 is dropped here
