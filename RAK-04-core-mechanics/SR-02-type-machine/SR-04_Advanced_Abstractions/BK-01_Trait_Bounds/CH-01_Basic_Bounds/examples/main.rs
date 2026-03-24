use std::fmt::Display;

fn main() {
    let m = Mobil { merk: String::from("Toyota") };
    info_display(m);
    
    info_display(77);
}

struct Mobil {
    merk: String,
}

// Mengimplementasikan Display agar memenuhi syarat bound
impl Display for Mobil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mobil merk {}", self.merk)
    }
}

// Fungsi dengan Trait Bound
fn info_display<T: Display>(item: T) {
    println!("Info: {}", item);
}
