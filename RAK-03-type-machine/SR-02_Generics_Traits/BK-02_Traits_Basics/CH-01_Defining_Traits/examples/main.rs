// Mendefinisikan Trait
trait Bersuara {
    fn keluarkan_suara(&self) -> String;
}

struct Bebek;
struct Kucing;

// Implementasi Trait untuk Bebek
impl Bersuara for Bebek {
    fn keluarkan_suara(&self) -> String {
        String::from("Kwak!")
    }
}

// Implementasi Trait untuk Kucing
impl Bersuara for Kucing {
    fn keluarkan_suara(&self) -> String {
        String::from("Meong!")
    }
}

fn main() {
    let donald = Bebek;
    let tom = Kucing;

    println!("Bebek bersuara: {}", donald.keluarkan_suara());
    println!("Kucing bersuara: {}", tom.keluarkan_suara());
}
