fn main() {
    let nomor = 6;

    // Rantai kondisi else if
    if nomor % 4 == 0 {
        println!("Nomor habis dibagi 4");
    } else if nomor % 3 == 0 {
        println!("Nomor habis dibagi 3");
    } else if nomor % 2 == 0 {
        println!("Nomor habis dibagi 2");
    } else {
        println!("Nomor tidak habis dibagi 4, 3, atau 2");
    }

    // CATATAN: Meskipun 6 habis dibagi 3 DAN 2, 
    // Rust hanya menjalankan blok kueri pertama yang benar (habis dibagi 3)
    // dan langsung keluar dari rantai if-else.
}
