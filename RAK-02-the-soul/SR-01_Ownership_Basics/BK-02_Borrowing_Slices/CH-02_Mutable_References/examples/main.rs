fn main() {
    let mut s = String::from("halo");

    // Multiple immutable borrows are allowed
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    // But we cannot have a mutable borrow while immutable ones are active
    // let r3 = &mut s; // This would cause a compiler error

    {
        // One mutable borrow is allowed if no others are active
        let r3 = &mut s;
        r3.push_str(", world");
        println!("Modified string: {}", r3);
    } // r3 goes out of scope here

    // Now we can borrow s again
    println!("Final string: {}", s);
}
