fn main() {
    let spaces = "   "; // Variabel pertama (String)
    
    // Shadowing: Mengubah tipe data dengan nama yang sama
    let spaces = spaces.len(); // Sekarang bertipe usize

    println!("Jumlah spasi: {}", spaces);

    // Tanpa shadowing, kita harus membuat nama baru seperti:
    // let spaces_str = "   ";
    // let spaces_len = spaces_str.len();
}
