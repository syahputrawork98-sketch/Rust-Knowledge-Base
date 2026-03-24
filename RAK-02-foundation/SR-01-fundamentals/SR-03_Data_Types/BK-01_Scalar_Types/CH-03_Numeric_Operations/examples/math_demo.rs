fn main() {
    // Penjumlahan & Pengurangan
    let sum = 10 + 20;
    let sub = 100 - 55;

    // Perkalian
    let mult = 10 * 10;

    // Masalah Pembagian Integer
    let int_div = 10 / 3; // Hasil: 3
    let float_div = 10.0 / 3.0; // Hasil: 3.333

    // Modulo (Sisa Bagi)
    let modulo = 10 % 3; // Hasil: 1

    println!("Penjumlahan (10 + 20): {}", sum);
    println!("Pengurangan (100 - 55): {}", sub);
    println!("Perkalian (10 * 10): {}", mult);
    println!("Pembagian Integer (10 / 3): {}", int_div);
    println!("Pembagian Float (10.0 / 3.0): {}", float_div);
    println!("Modulo (10 % 3): {}", modulo);

    // Casting (Mengubah Tipe)
    let a: i32 = 10;
    let b: f64 = 3.0;
    let result = (a as f64) / b; // a diubah ke f64 agar bisa dihitung dengan b
    println!("Hasil kalkulasi campuran (casting): {}", result);
}
