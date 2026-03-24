fn main() {
    let mut counter = 0;

    println!("Mulai berhitung...");

    // loop murni tanpa kondisi awal
    let hasil = loop {
        counter += 1;
        println!("Hitungan ke-{}", counter);

        if counter == 5 {
            // Berhenti dan melempar nilai counter * 10
            break counter * 10;
        }
    };

    println!("Loop berhenti. Hasil akhir (counter * 10): {}", hasil);

    // Tip: loop sangat berguna untuk me-retry operasi yang mungkin gagal
    // sampai operasi tersebut berhasil.
}
