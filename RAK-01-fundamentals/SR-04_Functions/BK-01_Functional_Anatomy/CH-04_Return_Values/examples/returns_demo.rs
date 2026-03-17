fn main() {
    let angka = 10;
    
    // Memanggil fungsi yang mengembalikan nilai
    let hasil_tambah = tambah_satu(angka);
    println!("{angka} ditambah 1 adalah {hasil_tambah}");

    let sapaan = buat_sapaan("Budi");
    println!("{sapaan}");

    let umur = 15;
    if boleh_masuk(umur) {
        println!("Selamat datang!");
    } else {
        println!("Maaf, Anda terlalu muda.");
    }
}

// 1. Fungsi sederhana dengan i32 return
fn tambah_satu(n: i32) -> i32 {
    n + 1 // Ekspresi akhir
}

// 2. Fungsi dengan String return
fn buat_sapaan(nama: &str) -> String {
    format!("Halo, {nama}! Selamat datang di dunia Rust.")
}

// 3. Fungsi dengan boolean dan logic internally
fn boleh_masuk(umur: i32) -> bool {
    if umur >= 17 {
        return true; // Explicit return untuk keperluan logika
    }
    
    false // Implicit return
}
