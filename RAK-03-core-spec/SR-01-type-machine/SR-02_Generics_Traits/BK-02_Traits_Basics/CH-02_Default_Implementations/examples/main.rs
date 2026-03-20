trait Info {
    // Default implementation
    fn metadata(&self) -> String {
        String::from("Objek Tanpa Nama")
    }
}

struct Robot {
    id: u32,
}

struct Manusia {
    nama: String,
}

// Robot menggunakan default
impl Info for Robot {}

// Manusia melakukan override
impl Info for Manusia {
    fn metadata(&self) -> String {
        format!("Manusia bernama: {}", self.nama)
    }
}

fn main() {
    let r1 = Robot { id: 1 };
    let m1 = Manusia { nama: String::from("Budi") };

    println!("Info Robot: {}", r1.metadata());
    println!("Info Manusia: {}", m1.metadata());
}
