# Panduan Terminologi (Rust Edition)

Menangkap ketajaman dan keamanan sistem melalui bahasa yang tepat sesuai dengan **Unified Gold Standard (PPM V4)**.

## 1. Aturan Penulisan Istilah
- **Precision Focus**: Gunakan kata kerja yang mendeskripsikan manajemen memori dan tipe data (e.g., *Borrowed*, *Moved*, *Monomorphized*, *Inlined*).
- **Spec-Sync**: Seluruh istilah teknis merujuk pada *Rust Reference* dan *The Rust Programming Language*.

## 2. Senior vs Basic Terms
Gunakan terminologi profesional untuk akurasi teknis:
- **Ownership & Borrowing** (bukan sekadar "penggunaan variabel").
- **Zero-Cost Abstractions** (bukan sekadar "fitur cepat").
- **Algebraic Data Types (ADTs)** (untuk Enums/Structs).
- **Affine Type System** (mekanika di balik move semantics).

## 3. Hierarchical Terminology (6-Level Standard)

| Term | Level | Analogi | Deskripsi |
| :--- | :--- | :--- | :--- |
| **Root** | 1 | Hub | Pusat navigasi utama repositori. |
| **RAK (Rack)**| 2 | Domain | Pengelompokan besar area ilmu (e.g., Anatomy, Foundation). |
| **SR (Sub-Rack)**| 3 | Track | Jalur spesifik di dalam domain (e.g., Fundamentals). |
| **BK (Book)** | 4 | Koleksi | Kumpulan bab yang membentuk satu modul utuh. |
| **CH (Chapter)**| 5 | Materi | Unit pengerjaan materi inti (Stage 1-5 PPM). |
| **SEC (Section)**| 6 | Detil | Granulitas terdalam untuk membedah sub-materi. |

---
*Referensi: [Architecture](./architecture.md) | [Conventions](./conventions.md)*
