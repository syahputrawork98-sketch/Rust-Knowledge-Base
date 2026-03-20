fn main() {
    let x = 5;
    let y = x; // COPY occurs (Stack data)

    println!("x: {}, y: {}", x, y); // x is still valid because it implements Copy
}
