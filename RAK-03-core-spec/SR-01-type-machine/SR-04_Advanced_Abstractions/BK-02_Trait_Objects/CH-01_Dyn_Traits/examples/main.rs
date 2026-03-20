trait Suara {
    fn bunyi(&self);
}

struct Anjing;
struct Kucing;

impl Suara for Anjing {
    fn bunyi(&self) { println!("Guk!"); }
}

impl Suara for Kucing {
    fn bunyi(&self) { println!("Meong!"); }
}

fn main() {
    // Vector berisi berbagai tipe yang mengimplementasikan Suara
    let daftar_hewan: Vec<Box<dyn Suara>> = vec![
        Box::new(Anjing),
        Box::new(Kucing),
        Box::new(Anjing),
    ];

    for hewan in daftar_hewan {
        hewan.bunyi();
    }
}
