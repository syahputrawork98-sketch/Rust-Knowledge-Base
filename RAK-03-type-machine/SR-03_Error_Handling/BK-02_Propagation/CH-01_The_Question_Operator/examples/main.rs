use std::fs::File;
use std::io::{self, Read};

fn main() {
    match jalankan_proses() {
        Ok(isi) => println!("Isi file: {}", isi),
        Err(e) => println!("Proses gagal di tengah jalan: {:?}", e),
    }
}

// Fungsi yang meneruskan error ke pemanggilnya
fn jalankan_proses() -> Result<String, io::Error> {
    // Membuka file (mungkin gagal)
    let mut f = File::open("test.txt")?;

    let mut isi = String::new();
    // Membaca file (mungkin gagal)
    f.read_to_string(&mut isi)?;

    // Jika sampai di sini, artinya semua proses di atas sukses
    Ok(isi)
}
