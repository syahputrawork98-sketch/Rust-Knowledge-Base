fn main() {
    enum LampuLaluLintas {
        Merah,
        Kuning,
        Hijau,
    }

    let status_sekarang = LampuLaluLintas::Merah;

    // Enum bisa dikirim ke fungsi
    cek_lampu(status_sekarang);
}

fn cek_lampu(lampu: LampuLaluLintas) {
    // Di bab selanjutnya kita akan belajar 'match' untuk memproses ini
    println!("Lampu sedang aktif...");
}
