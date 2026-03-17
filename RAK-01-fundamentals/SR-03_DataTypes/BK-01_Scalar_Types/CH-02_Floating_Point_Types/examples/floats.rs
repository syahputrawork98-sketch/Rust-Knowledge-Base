fn main() {
    // f64 adalah default
    let x = 3.141592653589793; 
    
    // f32 harus dideklarasikan eksplisit
    let y: f32 = 1.1234567;

    println!("Standard f64: {}", x);
    println!("Standard f32: {}", y);

    // Contoh perbedaan presisi jika angka terlalu panjang
    let presisi_tinggi: f64 = 0.1234567890123456789;
    let presisi_rendah: f32 = 0.1234567890123456789;

    println!("Tinggi (f64): {}", presisi_tinggi);
    println!("Rendah (f32): {}", presisi_rendah);
}
