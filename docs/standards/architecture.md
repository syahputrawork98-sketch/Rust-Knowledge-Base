# Arsitektur & Hierarki Struktur (Rust Edition)

Proyek **Rust Knowledge Base** disusun dengan satu standar tunggal: **Unified Gold Standard (PPM V4)**. Sistem ini memastikan keseimbangan antara narasi yang manusiawi dengan ketajaman teknis spesifikasi sistem.

## 1. Hirarki 6-Level (Universe Standard)

Sistem ini mengikuti hirarki kedalaman yang konsisten untuk seluruh repositori:

| Tingkatan | Nama | Analogi | Prefix | Contoh Direktori |
| :--- | :--- | :--- | :--- | :--- |
| **Level 1** | **Root** | Library Hub | `/` | `/` |
| **Level 2** | **Rak** | Domain Utama | `RAK-` | `RAK-01-anatomy/` |
| **Level 3** | **Sub-Rak** | Track Spesifik | `SR-` | `SR-01-fundamentals/` |
| **Level 4** | **Buku** | Koleksi Terpadu | `BK-` | `BK-01_Ownership/` |
| **Level 5** | **Bab** | Materi Inti | `CH-` | `CH-01_BorrowChecker/` |
| **Level 6** | **Section** | Detail Halaman | `SEC-` | `SEC-01_AliasingRules/` |

> [!IMPORTANT]
> **Pengecualian RAK-01 (Anatomy)**: 
> Khusus untuk RAK-01, hierarki menggunakan **Flat Structure** dengan melompati Level 3 (**SR-**). Struktur RAK-01 adalah: **RAK > BK > CH > SEC**.

---

## 2. Prinsip "Digital Mirroring"
Hierarki ini mencerminkan struktur sumber primer (The Rust Programming Language / Rust Reference) secara **1:1**. Jika sumber menuntut kedalaman lebih (misal: sub-clause spesifikasi), sistem akan menggunakan Level 6 (**SEC**) untuk menampung detail tersebut tanpa merusak struktur utama.

---

## 3. Karakteristik & Branding (7-RAK Architecture)

1. **RAK-01-anatomy**: Filosofi desain memori aman tanpa GC (The Landscape).
2. **RAK-02-foundation**: Sintaks dan tipe data dasar (The standard Book).
3. **RAK-03-evolution**: Kronologi rilis dan Rust Editions.
4. **RAK-04-core-mechanics**: Ownership, Borrowing, Lifetimes, Traits (The Internal Logic).
5. **RAK-05-ecosystem**: Cargo, std vs core, dan Crates.io.
6. **RAK-06-the-underworld**: Unsafe Rust, FFI, dan Memory Layout.
7. **RAK-07-specialization**: Async, Wasm, dan Embedded.

---

## 4. Kriteria Kelulusan Standar (Gold Standard)
Sebuah unit materi (CH/SEC) dianggap **Complete** jika menyajikan 5 representasi ini di dalam `README.md`:
1. **Source Link** (Akurasi Spec Rust Book / Reference).
2. **Dual Definition** (Definisi Formal + Analogi Model Mental).
3. **Mermaid Diagram inline** (Visualisasi aliran memori/eksekusi).
4. **Mekanisme Detil** (Penjelasan Compiler/Runtime behind-the-scene).
5. **Lab Praktis** (Penjelasan & relasi ke kode `.rs` fungsional di direktori `examples/`).

---
*Referensi: [Standard Workflow](./workflow.md) | [Conventions](./conventions.md)*
