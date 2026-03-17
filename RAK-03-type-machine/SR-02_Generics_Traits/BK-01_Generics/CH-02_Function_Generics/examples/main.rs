fn main() {
    let angka = vec![1, 2, 3];
    cetak_item(&angka);

    let teks = vec!["halo", "dunia"];
    cetak_item(&teks);
}

// Fungsi Generic sederhana
fn cetak_item<T: std::fmt::Debug>(list: &[T]) {
    for item in list {
        println!("{:?}", item);
    }
}
