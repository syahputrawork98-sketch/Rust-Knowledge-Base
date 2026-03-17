# Konvensi Penamaan & Struktur Folder

Setiap elemen dalam perpustakaan mengikuti aturan penamaan yang ketat untuk memudahkan navigasi AI dan Manusia.

## 1. Aturan Penamaan Folder
- **Prefix Kapital**: Gunakan prefix sesuai levelnya (`RAK-`, `SR-`, `BK-`, `CH-`).
- **Nomor Urut**: Gunakan dua digit untuk nomor urut (`01`, `02`, dst).
- **Snake_Case**: Nama deskriptif menggunakan snake_case setelah prefix (misal: `SR-01_MemorySafety`).

## 2. Struktur Internal Bab (CH-XX)
Setiap folder bab harus memiliki:
- `README.md`: Berisi narasi, analogi, dan penjelasan teknis.
- `examples/`: Berisi file `.rs` yang valid dan bisa dijalankan untuk demonstrasi materi.

## 3. Penulisan File
- Gunakan Markdown untuk dokumentasi.
- Diagram wajib menggunakan Mermaid.js.
- Referensi dokumen resmi Rust (The Book/Reference) harus disertakan jika relevan.

---
*Back to [Structure Guide](../structure-guide.md)*
