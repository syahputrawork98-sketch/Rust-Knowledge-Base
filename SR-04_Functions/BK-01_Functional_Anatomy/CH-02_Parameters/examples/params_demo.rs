fn main() {
    // Memberikan argumen yang sesuai dengan tipe parameter
    tambah_dan_cetak(10, 20);
    
    let pesan = "Status sistem: ";
    cetak_status(pesan, true);
}

// Parameter x dan y bertipe i32
fn tambah_dan_cetak(x: i32, y: i32) {
    let hasil = x + y;
    println!("Hasil penjumlahan {x} + {y} = {hasil}");
}

// Parameter label bertipe &str, is_active bertipe bool
fn cetak_status(label: &str, is_active: bool) {
    println!("{label} {}", if is_active { "AKTIF" } else { "NONAKTIF" });
}
