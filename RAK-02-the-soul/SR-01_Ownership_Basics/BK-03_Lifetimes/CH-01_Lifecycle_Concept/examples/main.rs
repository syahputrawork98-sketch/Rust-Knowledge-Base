fn main() {
    let x = 5;            // Lifetime 'x starts
    {
        let r = &x;       // Lifetime 'r starts
        println!("r: {}", r);
    }                     // Lifetime 'r ends
}                         // Lifetime 'x ends
