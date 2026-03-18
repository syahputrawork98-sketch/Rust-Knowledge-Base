# Panduan Terminologi (Rust Edition)

Mencerminkan disiplin, keamanan, dan performa tanpa rasa takut.

## 1. Aturan Penulisan Istilah
- **Safety First**: Fokus pada istilah yang berkaitan dengan kepemilikan dan keamanan data.
- **No Compromise**: Tidak ada jalan pintas; jelaskan aturan compiler dengan tegas.

## 2. Senior vs Basic Terms

| Basic Term | Senior Terminology | Konteks |
| :--- | :--- | :--- |
| Aturan pinjam | **Borrowing & Lifetimes** | Manajemen memori kompilasi. |
| Konkurensi aman | **Fearless Concurrency** | Sifat Thread-safety Rust. |
| Fitur tambahan | **Traits** | Abstraksi perilaku objek. |
| Kepemilikan data | **Ownership** | Konsep dasar manajemen memori. |

## 3. Metode Analogi
- **The Fearless Guardian**: Bayangkan Compiler Rust sebagai penjaga gerbang yang tegas namun baik hati demi keselamatan Anda.
- **Buku Catatan Pustakawan**: Anggap *Borrow Checker* sebagai pustakawan yang mencatat siapa yang meminjam buku dan kapan harus dikembalikan.
