fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    
    // Slices are references, so they are efficient
    println!("First word: {}", hello);
    println!("Second word: {}", world);

    // String literals like this are also string slices (&str)
    let literal = "foo bar";
    let foo = &literal[0..3];
    println!("Literal slice: {}", foo);
}
