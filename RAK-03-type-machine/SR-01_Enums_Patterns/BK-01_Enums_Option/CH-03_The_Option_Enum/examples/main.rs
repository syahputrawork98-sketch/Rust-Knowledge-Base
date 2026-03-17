fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // rustc tidak akan mengizinkan baris ini:
    // let sum = x + y; 

    // Kita harus mengecek isinya:
    match y {
        Some(nilai) => println!("Hasil tambah: {}", x + nilai),
        None => println!("Tidak bisa dijumlahkan karena kosong!"),
    }
}
