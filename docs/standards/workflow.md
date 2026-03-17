# Alur Kerja & Prosedur Penulisan (PPM)

Penulisan materi di Rust Knowledge Base mengikuti standar kualitas tinggi yang disebut **"Gold Standard"**. Setiap Bab (CH) harus memiliki empat komponen wajib:

## 1. Narasi Detail (The Narrative)
Jangan hanya menyalin dokumentasi teknis. Gunakan pendekatan bercerita yang ramah terhadap "orang awam" namun tetap akurat. 

- **Definisi Judul (Title Definition)**: **WAJIB** menjelaskan secara gamblang apa arti dari judul bab tersebut di awal narasi. Misalnya, jika judulnya "Installing Rust", jelaskan apa itu proses instalasi dalam konteks sistem Rust. Jika judulnya "The Entry Point", jelaskan apa itu titik masuk sebuah program.
- **Kaidah Istilah**: Narasi **WAJIB** menggunakan istilah teknis resmi sesuai *The Rust Book* (e.g., *Ownership*, *Borrowing*, *Shadowing*). Hal ini penting agar "orang awam" memiliki bahasa yang sama saat berdiskusi dengan *Senior Developer*, menghindari miskomunikasi atau "lost context".
- **Kaidah "Kenapa"**: Penjelasan harus menjawab "Kenapa" fitur tersebut ada, bukan sekadar "Bagaimana" cara menggunakannya.

## 2. Dua Analogi (The Dual Analogies)
Analogisasi adalah kunci untuk membedah konsep sistem Rust yang kompleks:
- **Analogi Singkat (Short Analogy)**: Penjelasan 1-2 kalimat untuk pemahaman instan (e.g., "rustup" seperti asisten pribadi).
- **Analogi Panjang (Long Analogy)**: Konteks mendalam yang membangun gambaran sistem secara utuh (e.g., analogi bengkel kayu untuk toolchain).

## 3. Visualisasi SVG/Mermaid (The Map)
Gunakan diagram untuk memetakan alur kerja, struktur memori (Stack vs Heap), atau hubungan antar modul.
- Semua diagram harus menggunakan format **Mermaid.js** atau **SVG**.
- Diagram harus interaktif atau menyertakan Emoji agar lebih hidup.

## 4. Contoh Kode Praktis (The Examples)
Setiap Bab harus memiliki folder `examples/` berisi minimal satu file `.rs` yang bisa dijalankan.
- Kode harus fokus, bersih, dan menyertakan komentar penjelasan di baris-baris kritis.

---

## 📅 Urutan Pengerjaan (Creation Sequence)
Untuk menjaga konsistensi dan akurasi, pengerjaan bab **WAJIB** mengikuti urutan berikut:

1.  **Narasi & Analogi**: Membangun fondasi pemahaman dan mental model terlebih dahulu.
2.  **Contoh Kode (Examples)**: Mengimplementasikan teori ke dalam kode yang bisa dijalankan.
3.  **Visualisasi (SVG/Mermaid)**: Dibuat terakhir untuk memvisualisasikan narasi dan kode yang sudah matang. 

*Catatan: Gambar/Visualisasi harus mencerminkan apa yang sudah dijelaskan di narasi dan contoh kode.*

---
*Back to [Structure Guide](../structure-guide.md)*
