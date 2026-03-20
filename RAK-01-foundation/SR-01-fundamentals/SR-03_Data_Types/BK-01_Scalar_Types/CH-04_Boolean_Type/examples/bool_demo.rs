fn main() {
    let t = true;
    let f = false;

    println!("Kondisi benar: {}", t);
    println!("Kondisi salah: {}", f);

    // Operasi Logika
    let dan = t && f; // false
    let atau = t || f; // true
    let tidak = !t;    // false

    println!("Logika AND: {}", dan);
    println!("Logika OR: {}", atau);
    println!("Logika NOT: {}", tidak);

    // Penggunaan umum: if condition
    if t {
        println!("Gerbang terbuka!");
    }
}
