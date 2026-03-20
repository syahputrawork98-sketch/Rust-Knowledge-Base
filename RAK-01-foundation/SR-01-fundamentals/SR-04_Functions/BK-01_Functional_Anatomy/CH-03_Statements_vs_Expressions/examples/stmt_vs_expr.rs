fn main() {
    // 1. Statemen: Definisi variabel
    let x = 5;

    // 2. Ekspresi: Penjumlahan
    // Nilai ini bisa ditampung ke variabel lain
    let y = x + 10; 

    // 3. Blok Ekspresi:
    // Seluruh isi kurung kurawal dievaluasi dan "melempar" keluar nilai terakhir
    let score = {
        let bonus = 100;
        bonus * 2 // Tanpa titik koma, nilai 200 dilempar keluar
    };

    println!("Nilai y: {y}");
    println!("Nilai score: {score}");

    // CONTOH SALAH: Jika kita tambah titik koma di bonus * 2, 
    // maka score akan bernilai () dan program tidak akan jalan jika kita berharap angka.
}
