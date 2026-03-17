fn main() {
    let koin_beruntung = Some(7);

    // if let adalah shorthand untuk match Some(...)
    if let Some(angka) = koin_beruntung {
        println!("Anda mendapatkan angka keberuntungan: {}", angka);
    } else {
        println!("Sayang sekali, kotak kado kosong.");
    }

    // if let dengan Enum custom
    enum Status {
        Aktif(u32),
        Nonaktif,
    }

    let user_status = Status::Aktif(100);

    if let Status::Aktif(skor) = user_status {
        println!("User sedang aktif dengan skor: {}", skor);
    }
}
