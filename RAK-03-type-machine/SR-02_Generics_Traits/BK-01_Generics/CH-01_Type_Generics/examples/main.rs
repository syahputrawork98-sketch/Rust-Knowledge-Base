struct Titik<T> {
    x: T,
    y: T,
}

impl<T> Titik<T> {
    fn ambil_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let t1 = Titik { x: 5, y: 10 };
    println!("t1.x = {}", t1.ambil_x());

    let t2 = Titik { x: 1.1, y: 4.4 };
    println!("t2.x = {}", t2.ambil_x());
}
