fn main() {
    // Definisi awal: x punya izin untuk berubah (mut)
    let mut x = 5;
    println!("Nilai awal: {}", x);

    // Mengubah nilai
    x = 10;
    println!("Nilai setelah diubah: {}", x);

    // Menambah nilai
    x = x + 1;
    println!("Nilai setelah ditambah 1: {}", x);
}
