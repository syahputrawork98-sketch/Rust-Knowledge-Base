fn main() {
    // 1. Membuat Tuple (Signed, Float, Unsigned)
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 2. Destructuring (Membongkar isinya ke variabel baru)
    let (x, y, z) = tup;
    println!("Hasil destructuring - y: {}", y);

    // 3. Akses via Index (.)
    let lima_ratus = tup.0;
    println!("Akses index 0 (.0): {}", lima_ratus);

    // 4. Tuples juga bisa mengandung tipe yang sama
    let kordinat: (i32, i32) = (10, 20);
    println!("Kordinat: ({}, {})", kordinat.0, kordinat.1);

    // 5. Unit Type (Tuple Kosong)
    let kosong: () = ();
    println!("Ini adalah unit type (kosong): {:?}", kosong);
}
