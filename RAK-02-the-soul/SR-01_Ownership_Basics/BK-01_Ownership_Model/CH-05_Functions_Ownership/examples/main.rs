fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // ERROR: s moved into function

    let x = 5;
    makes_copy(x);
    println!("x is still valid: {}", x);

    let s1 = gives_ownership();
    println!("Got s1: {}", s1);
}

fn takes_ownership(some_string: String) {
    println!("Received ownership: {}", some_string);
} // some_string is dropped here

fn makes_copy(some_integer: i32) {
    println!("Received copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // ownership is moved to caller
}
