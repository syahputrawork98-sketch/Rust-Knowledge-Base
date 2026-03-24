# Konvensi Penamaan & Struktur Folder (Rust Edition)

Untuk menjaga keteraturan "Perpustakaan Digital", setiap file dan folder wajib mengikuti konvensi penamaan berikut sesuai dengan **Unified Gold Standard (PPM V4)**.

## 1. Penamaan Direktori

| Level | Prefix | Format | Contoh |
| :--- | :--- | :--- | :--- |
| **Rak** | `RAK-` | `RAK-XX-slug` | `RAK-01-anatomy` |
| **Sub-Rak** | `SR-` | `SR-XX-slug` | `SR-01-fundamentals` |
| **Buku** | `BK-` | `BK-XX_Slug` | `BK-01_Ownership` |
| **Bab** | `CH-` | `CH-XX_Slug` | `CH-01_BorrowChecker` |
| **Section** | `SEC-` | `SEC-XX_Slug` | `SEC-01_AliasingRules` |

> [!IMPORTANT]
> **Rak dan Sub-Rak** menggunakan tanda hubung (`-`) untuk slug. 
> **Buku, Bab, dan Section** menggunakan garis bawah (`_`) untuk memisahkan urutan dengan nama slug.

---

## 2. Struktur Internal Unit (Level 5 & 6)

Setiap folder Bab (**CH**) atau Section (**SEC**) memiliki struktur dasar berikut:
```text
CH- atau SEC-/
├── README.md        <- Materi teks inti berserta diagram Mermaid inline.
├── examples/        <- Kode lab fungsional multi-file (.rs).
└── assets/          <- Arsip media statis (svg/png).
```

> [!CAUTION]
> **Aturan "Nil Content" (Murni Narasi)**:
> Jika sebuah unit secara spesifik bersifat sejarah/filosofis (Nil Content), Anda **DILARANG** membuat direktori `examples/` maupun `assets/`. Biarkan folder unit tersebut hanya berisi `README.md`.

---

## 3. Aturan Penamaan Lab Praktis (`examples/`)
File di dalam direktori `examples/` **WAJIB** menggunakan *prefix* numerik berurutan.
**Contoh Benar**: `01_basic_syntax.rs`, `02_ownership_transfer.rs`, `03_borrowing_rules.rs`.
**Contoh Salah**: `main.rs`, `testing.rs`, `contoh.rs`.

---

## 4. Aturan README.md
Setiap tingkatan dari Level 2 hingga Level 6 wajib memiliki file `README.md` sebagai hub navigasi atau penyaji materi.

---
*Referensi: [Architecture](./architecture.md) | [Workflow](./workflow.md)*
