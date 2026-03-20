fn main() {
    // Secara default, x bersifat immutable (tidak bisa diubah)
    let x = 5;
    println!("Nilai x awal: {}", x);

    // Baris di bawah ini akan menyebabkan error kompilasi
    // x = 6; 
    
    println!("Jika kita ingin mengubah x, kita butuh kata kunci 'mut'.");
}
