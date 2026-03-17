# Arsitektur & Hierarki (PPM)

Sistem **PPM (Perpustakaan Pribadi Modular)** menggunakan hierarki 5-level untuk memastikan setiap informasi memiliki tempat yang logis dan mudah ditemukan.

## 1. Rak (Rack) - Level 1
Kelompok tema besar bahasa Rust.
- **Folder**: `RAK-01-fundamentals`, `RAK-02-advanced`, dsb.
- **Wajib**: `README.md` yang merangkum isi Rak.

## 2. Sub-Rak (Sub-Rack) - Level 2
Pembagian granular berdasarkan domain teknis (misal: Memory Safety, Types, Concurrency).
- **Folder**: `SR-01_MemorySafety`, `SR-02_Concurrency`, dsb.
- **Wajib**: `README.md` yang mencantumkan daftar Buku di dalamnya.

## 3. Buku (Book) - Level 3
Unit pembelajaran tematik dalam satu Sub-Rak.
- **Folder**: `BK-01_OwnershipModel`, `BK-02_BorrowingMechanics`, dsb.
- **Wajib**: `README.md` sebagai daftar isi Bab.

## 4. Bab (Chapter) - Level 4
Unit atomik dari materi.
- **Folder**: `CH-01_Overview`, `CH-02_DeepDive`, dsb.
- **Wajib**: `README.md` (Materi) dan folder `examples/` (Kode).

## 5. Section - Level 5
Sub-topik di dalam satu README Bab (Header H2 atau H3).
- Memberikan detail spesifik tentang mekanisme tertentu.

---
*Back to [Structure Guide](../structure-guide.md)*
