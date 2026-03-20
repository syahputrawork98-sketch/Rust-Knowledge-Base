fn main() {
    let a = 'a';
    let b: char = 'B';
    let g: char = 'G';
    
    // Karakter Unicode Non-Latin
    let hobi: char = '📚';
    let salam: char = '你好'; // Ini ERROR jika ditaruh di char tunggal (lebih dari 1 char)
    let oke = '👌';

    println!("Alfabet: {} {} {}", a, b, g);
    println!("Emoji Hobi: {}", hobi);
    println!("Emoji Oke: {}", oke);

    // Iterasi char dalam string (preview)
    let pesan = "Rust";
    println!("Mengkoleksi karakter dari '{}':", pesan);
    for c in pesan.chars() {
        println!(" - {}", c);
    }
}
