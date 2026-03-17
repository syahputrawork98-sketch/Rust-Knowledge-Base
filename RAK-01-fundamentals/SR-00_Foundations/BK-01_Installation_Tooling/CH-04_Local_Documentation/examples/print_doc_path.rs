use std::process::Command;

fn main() {
    println!("--- Cek Lokasi Dokumentasi Lokal ---");

    // Perintah untuk menampilkan path dokumentasi
    let output = Command::new("rustup")
        .args(&["doc", "--path"])
        .output();

    match output {
        Ok(out) => {
            let path_str = String::from_utf8_lossy(&out.stdout);
            if !path_str.is_empty() {
                println!("Dokumentasi Anda tersimpan di:");
                println!("{}", path_str.trim());
                println!("\nGunakan 'rustup doc' untuk membukanya di browser.");
            } else {
                println!("[!] Gagal mendapatkan path. Pastikan Rust sudah terinstal.");
            }
        }
        Err(_) => println!("[FAIL] Perintah 'rustup' tidak ditemukan."),
    }
}
