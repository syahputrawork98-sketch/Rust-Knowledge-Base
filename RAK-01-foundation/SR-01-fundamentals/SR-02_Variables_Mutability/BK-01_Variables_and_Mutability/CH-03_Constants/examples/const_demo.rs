// Konstanta Global
const MAX_POINTS: u32 = 100_000;

fn main() {
    // Konstanta Lokal
    const SPEED_OF_LIGHT: u32 = 299_792_458;

    println!("Batas poin maksimal: {}", MAX_POINTS);
    println!("Kecepatan cahaya: {} m/s", SPEED_OF_LIGHT);

    // MAX_POINTS = 200_000; // ❌ ERROR: Konstanta tidak bisa diubah nilainya.
}
