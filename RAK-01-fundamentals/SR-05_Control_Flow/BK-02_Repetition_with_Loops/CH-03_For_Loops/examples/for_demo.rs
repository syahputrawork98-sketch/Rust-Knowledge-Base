fn main() {
    // 1. Iterasi Array (Cara paling umum)
    let bumbu_dapur = ["Garam", "Gula", "Merica", "Ketumbar"];

    println!("Daftar bumbu:");
    for bumbu in bumbu_dapur {
        println!("- {}", bumbu);
    }

    println!("\n--- Peluncuran Roket ---");

    // 2. Iterasi Range (Eksklusif: 1 sampai 3)
    // .rev() digunakan untuk membalik urutan (Count Down)
    for hitungan_mundur in (1..4).rev() {
        println!("{}", hitungan_mundur);
    }

    println!("TAKE OFF!!! 🚀");
}
