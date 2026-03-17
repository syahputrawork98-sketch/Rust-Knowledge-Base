fn main() {
    let nomor = 3;

    // Contoh condition boolean murni
    if nomor < 5 {
        println!("Angka {} lebih kecil dari 5", nomor);
    } else {
        println!("Angka {} lebih besar atau sama dengan 5", nomor);
    }

    // Contoh pengecekan ketidaksamaan (bool expression)
    let lapar = true;
    if lapar {
        println!("Waktunya makan!");
    }

    // CATATAN: Rust tidak bisa melakukan if nomor { ... } 
    // karena nomor adalah integer, bukan boolean.
}
