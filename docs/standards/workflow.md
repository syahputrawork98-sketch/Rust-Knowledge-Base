# Alur Kerja Penulisan (PPM) V4 - Rust Edition

Prosedur Penulisan Materi (PPM) memastikan setiap Bab (**CH**) atau Section (**SEC**) di **Rust Knowledge Base** memiliki kualitas yang setara dengan repositori "Gold Standard".

## Tahapan PPM (Prosedur Penulisan Materi) V4

Setiap penyusunan `README.md` unit materi (CH/SEC) wajib disusun dengan **urutan representasi** 5 Tahapan (Gold Standard) berikut secara disiplin:

### 1. Tahap 1: Source Alignments & Judul
- **Header**: Judul materi yang diiringi dengan metafora/analogi singkat (e.g., Ownership: The Memory Passport).
- **Source Link**: Tautan spesifik langsung ke *The Rust Programming Language*, *Rust Reference*, atau *RFC* yang relevan.

### 2. Tahap 2: Konsep & Esensi (Definisi & Rasionalitas)
- **Definisi ("Apa itu?")**: Menjelaskan konsep dasar secara formal (e.g., Affine Types, Move Semantics).
- **Rasionalitas ("Why & How?")**: Membedah alasan Rust menerapkan aturan tersebut (e.g., Mencegah Data Races / Double Free).
- **Analogi Model Mental**: Menjabarkannya dalam contoh dunia nyata.
- **Terminologi Teknis**: Istilah profesional (e.g., Monomorphization, RAII, Sized Trait).

### 3. Tahap 3: Visualisasi Sistem (Mermaid)
- **Inline Diagram**: Diagram Mermaid (seperti *Memory Layout*, *Borrow Checker State*, *Thread Synchronization*) diletakkan secara **inline** (````mermaid````) tepat setelah penjelasan konsep.
- **Fokus Visual**: Memfokuskan pada alokasi Stack vs Heap, pergerakan Ownership, atau transisi status data.

### 4. Tahap 4: Mekanisme Pembuktian (Deep-Dive)
- **Detail Balik Layar**: Menguliti bagaimana kode diproses oleh `rustc` (HIR -> MIR -> LLVM IR) dan perilaku runtime (jika ada, seperti `std::sync`).

### 5. Tahap 5: Multi-file Lab Praktis (Examples)
- **Kewajiban Referensi**: Setiap teori diakhiri dengan tautan ke lab praktik.
- **Struktur Folder `examples/`**: Berisi skrip `.rs` berurutan (e.g., `01_safe_move.rs`, `02_invalid_borrow.rs`). Setiap skrip harus memicu pemahaman (bisa berupa kode yang sengaja error untuk menunjukkan perilaku Borrow Checker).

---

### Pengecualian Eksplisit "Nil Content" (Narasi Sejarah)
Jika suatu unit murni bersifat historis atau konseptual non-teknis:
- **Tulis Penafian**: Wajib menyertakan teks *"Unit ini tidak membutuhkan Lab Praktis/Visualisasi karena bersifat penjelasan sejarah/konsep naratif."*
- **Pelarangan Direktori**: **DILARANG** membuat direktori `assets/` maupun `examples/` untuk menjaga kebersihan repositori.

---
*Target Akhir: Mencapai [Gold Standard](./architecture.md#kriteria-gold-standard).*
