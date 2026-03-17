#[derive(Debug)]
enum VersiIP {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = VersiIP::V4(127, 0, 0, 1);
    let loopback = VersiIP::V6(String::from("::1"));

    println!("Alamat rumah: {:?}", home);
    println!("Alamat loopback: {:?}", loopback);
}
