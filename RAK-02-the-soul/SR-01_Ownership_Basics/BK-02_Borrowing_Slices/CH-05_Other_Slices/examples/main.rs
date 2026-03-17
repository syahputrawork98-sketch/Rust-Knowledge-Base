fn main() {
    let a = [10, 20, 30, 40, 50];

    // Slicing an array
    let slice = &a[1..3];

    assert_eq!(slice, &[20, 30]);
    println!("Array slice: {:?}", slice);

    // Slices on vectors (similar syntax)
    let v = vec![1, 2, 3, 4, 5];
    let v_slice = &v[2..4];
    println!("Vector slice: {:?}", v_slice);
}
