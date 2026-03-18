# Rust Knowledge Base: Documentation Hub

Pusat dokumentasi ini menyimpan seluruh cetak biru, standar kualitas, dan panduan meta untuk membangun repository ini secara konsisten mengikuti standar *The Rust Bookshelf* (Official Docs).

## Daftar Dokumen Utama

### 1. Standar & Protokol
- **[Architecture Standards](./standards/architecture.md)**: Analogi "The Rust Bookshelf" dan kriteria Gold Standard.
- **[Naming Conventions](./standards/conventions.md)**: Aturan penamaan RAK, SR, BK, hingga Level Bab.
- **[PPM Workflow](./standards/workflow.md)**: 4 Tahapan Penulisan Materi (PPM) V4.
- **[Status Protocol](./standards/status-protocol.md)**: Cara melaporkan dan menghitung progress pengerjaan.
- **[Contribution Guide](./standards/contribution.md)**: Panduan kontribusi dan aturan PR.
- **[Core Contribution](./standards/core-contribution.md)**: Standar teknis kontribusi materi inti.

### 2. Cetak Biru (Blueprints)
- **[Repository Plan](./repository-plan/README.md)**: Dekomposisi total dari ekosistem Rust ke dalam struktur 10-Rack.

### 3. Narasi & Esensi
- **[Rust Origins](./rust-origins.md)**: Jejak langkah dari proyek personal Mozilla hingga standar kernel Linux.
- **[Rust History](./rust-history.md)**: Sejarah kelahiran dan evolusi Rust Edition.
- **[Philosophy & Essence](./rust-philosophy.md)**: Membedah Ownership, Borrowing, dan "Fearless Concurrency".
- **[Why Rust?](./why-rust.md)**: Rasionalitas teknis memilih keamanan memori tanpa mengorbankan kecepatan.

## Struktur Direktori `docs/`

```text
/docs
├── standards/             # Standar & Protokol (Architecture, Conventions, etc.)
├── repository-plan/       # Cetak biru 10-Rack (Level 2)
├── rust-history.md        # Narasi Sejarah
├── rust-philosophy.md     # Filosofi Bahasa
├── rust-origins.md        # Legenda Inisiasi
├── why-rust.md            # Rasionalitas Penggunaan
└── README.md              # File ini (Hub navigasi)
```
