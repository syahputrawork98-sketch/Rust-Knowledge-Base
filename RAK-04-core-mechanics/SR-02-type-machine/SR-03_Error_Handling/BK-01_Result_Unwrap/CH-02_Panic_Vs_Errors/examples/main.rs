fn main() {
    // Penggunaan panic! untuk kondisi yang benar-benar tidak boleh terjadi
    let skenario_kiamat = false;

    if skenario_kiamat {
        panic!("Sistem fail-safe diaktifkan! Menghentikan semua proses.");
    }

    // Penggunaan Result untuk input yang mungkin salah
    let input_user = "bukan_angka";
    let hasil: Result<i32, _> = input_user.parse();

    match hasil {
        Ok(n) => println!("Angka: {}", n),
        Err(_) => println!("Peringatan: Input Anda bukan angka yang valid! (Recovered)"),
    }
}
