fn main() {
    // String literals have 'static lifetime implicitly
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    // You can also use 'static for global variables
    const TITLE: &'static str = "Rust Documentation";
    println!("Title: {}", TITLE);
}
