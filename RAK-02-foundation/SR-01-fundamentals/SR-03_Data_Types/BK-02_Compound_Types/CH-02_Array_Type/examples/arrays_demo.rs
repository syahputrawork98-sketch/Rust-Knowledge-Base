fn main() {
    // 1. Membuat Array (Semua i32)
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 2. Pendeklarasian Array Pendamping (Inisialisasi Nilai Sama)
    let b = [3; 5]; // [3, 3, 3, 3, 3]

    // 3. Akses via Index []
    let pertama = a[0];
    let terakhir = a[4];

    println!("Array a: {:?}", a);
    println!("Array b (semua 3): {:?}", b);
    println!("Elemen index 0: {}", pertama);
    println!("Elemen index 4: {}", terakhir);

    // 4. Update data (Syarat: Harus mut)
    let mut c = [10, 20, 30];
    c[0] = 100;
    println!("Array c setelah diubah: {:?}", c);
}
