fn main() {
    // Bilangan bulat positif & negatif
    let x: i32 = -500;
    
    // Bilangan bulat hanya positif
    let y: u32 = 1000;

    // Penggunaan garis bawah (underscore) untuk angka besar (visual sugar)
    let satu_miliar: i64 = 1_000_000_000;

    println!("Signed: {}, Unsigned: {}", x, y);
    println!("Satu Miliar: {}", satu_miliar);

    // Contoh penentuan arsitektur
    let index: usize = 0; // Sering digunakan untuk index array/koleksi
    println!("Index awal: {}", index);
}
