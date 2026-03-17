fn main() {
    let s1 = String::from("halo");

    // Borrowing s1 by sending a reference (&s1)
    let len = calculate_length(&s1);

    // s1 is still valid here because we only borrowed it
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
