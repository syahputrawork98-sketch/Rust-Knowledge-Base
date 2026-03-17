fn main() {
    // Destructuring Tuple
    let kordinat = (10, 20, 30);
    let (x, _, z) = kordinat; // Mengabaikan nilai tengah
    println!("x: {}, z: {}", x, z);

    // Range Matching
    let char = 'c';
    match char {
        'a'..='j' => println!("Huruf awal alfabet"),
        'k'..='z' => println!("Huruf akhir alfabet"),
        _ => println!("Bukan alfabet kecil"),
    }

    // Multiple Matching
    let x = 1;
    match x {
        1 | 2 => println!("Satu atau Dua"),
        _ => println!("Lainnya"),
    }
}
