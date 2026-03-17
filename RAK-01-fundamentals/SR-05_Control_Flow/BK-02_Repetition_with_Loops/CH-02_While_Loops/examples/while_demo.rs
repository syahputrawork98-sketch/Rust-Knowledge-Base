fn main() {
    let mut stok_kopi = 5;

    println!("Stok kopi saat ini: {}", stok_kopi);

    // Berjalan selama stok_kopi lebih besar dari 0
    while stok_kopi > 0 {
        println!("Melayani pelanggan... Sisa stok: {}", stok_kopi);
        
        stok_kopi -= 1; // Penting: Ubah nilai agar tidak infinite loop
    }

    println!("Maaf, stok kopi habis. Kedai tutup.");
}
