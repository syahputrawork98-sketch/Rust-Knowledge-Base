fn main() {
    println!("--- Program Start ---");

    // Rust tidak keberatan fungsi didefinisikan di bawah
    sapa_dunia();

    println!("--- Program End ---");
}

// Menggunakan snake_case
fn sapa_dunia() {
    println!("Hello, World! Ini dipanggil dari sebuah fungsi.");
    
    // Fungsi juga bisa memanggil fungsi lainnya
    berikan_tips_rust();
}

fn berikan_tips_rust() {
    println!("Tip: Gunakan rust-analyzer untuk bantuan visual di IDE.");
}
