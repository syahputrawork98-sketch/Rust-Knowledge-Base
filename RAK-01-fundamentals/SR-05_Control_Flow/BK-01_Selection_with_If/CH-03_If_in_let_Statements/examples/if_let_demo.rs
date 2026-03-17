fn main() {
    let cuaca_panas = true;

    // Menugaskan nilai berdasarkan kondisi
    // KEDUA cabang harus mengembalikan tipe data yang sama (dalam hal ini &str)
    let minuman = if cuaca_panas {
        "Es Teh Manis"
    } else {
        "Kopi Hangat"
    };

    println!("Cuaca {} panas, aku pilih: {}", if cuaca_panas {"sedang"} else {"tidak"}, minuman);

    // Contoh dengan angka
    let skor = 85;
    let status = if skor >= 75 { 1 } else { 0 }; // 1 untuk lulus, 0 untuk gagal

    println!("Kode status kelulusan: {}", status);
}
