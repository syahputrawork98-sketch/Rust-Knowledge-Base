fn main() {
    // let reference_to_nothing = dangle();
    
    // Correct way: Move the ownership out of the function
    let healthy_string = no_dangle();
    println!("Result: {}", healthy_string);
}

/* 
// This function would fail to compile because it tries to return a reference
// to a value that will be dropped at the end of the function.
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership is moved out
}
