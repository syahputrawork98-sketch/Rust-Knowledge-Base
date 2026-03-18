# Rust Knowledge Base: Total Deconstruction Plan

> **Status Spec-Sync**: v2024 Edition (Full Alignment)
> **Last Updated**: 2026-03-19

Arsitektur **Source-Driven 10-Rack** ini mencerminkan taksonomi asli [doc.rust-lang.org](https://doc.rust-lang.org/).

---

## 🏗 Justifikasi Teknis (The Mirroring Principle)

Setiap Rak dipetakan langsung ke "Bookshelf" atau referensi utama di ekosistem dokumentasi Rust.

### 1. RAK-01: Getting Started
Instalasi `rustup`, konfigurasi toolchain, dan "Hello, Cargo!".

### 2. RAK-02: The Rust Programming Language
Panduan naratif utama (The Book). Jantung dari kurikulum Rust.

### 3. RAK-03: Rust Reference
Spesifikasi formal bahasa, membedah detail sintaksis dan perilaku compiler.

### 4. RAK-04: Standard Library API
Referensi API untuk crate `std`, `core`, dan `alloc`.

### 5. RAK-05: The Cargo Book
Manajemen paket, dependensi, build scripts, dan publikasi ke `crates.io`.

### 6. RAK-06: The rustc & rustdoc Books
Panduan penggunaan compiler (`rustc`) dan generator dokumentasi (`rustdoc`).

### 7. RAK-07: The Edition Guide
Membedah perbedaan antara edisi Rust (2015, 2018, 2021, 2024).

### 8. RAK-08: The Nomicon & Unstable Book
Dunia "Unsafe Rust", internal compiler, dan fitur malam (nightly features).

### 9. RAK-09: Rust by Example
Tutorial berbasis contoh kode untuk percepatan pemahaman praktis.

### 10. RAK-10: Specialized Books
Materi khusus seperti Embedded Rust, WebAssembly, dan Rust on CLI.

---

## 🗄 Peta Arsitektur Detail

| Rak | Sub-Rak (SR) | Buku (BK) | Deskripsi BK |
| :--- | :--- | :--- | :--- |
| **RAK-01** | SR-01: Setup | BK-01: Rustup | Managing versions. |
| **RAK-02** | SR-01: Soul | BK-01: Ownership | Ownership, Borrowing, Lifetimes. |
| | | BK-02: Smart Pointers | `Box`, `Rc`, `Arc`, `RefCell`. |
| **RAK-03** | SR-01: Specs | BK-01: Type Machine | Traits, Generics, Supertypes. |
| **RAK-04** | SR-01: Std Lib | BK-01: Core Types | `Option`, `Result`, `String`, `Vector`. |
| **RAK-05** | SR-01: Ecosystem | BK-01: Cargo logic | `Cargo.toml` & Semantic Versioning. |
| **RAK-06** | SR-01: Toolchain | BK-01: Linting | Clippy & Rustfmt usage. |
| **RAK-07** | SR-01: Timeline | BK-01: Rust 2024 | What's new in the latest edition. |
| **RAK-08** | SR-01: Advanced | BK-01: Unsafe Rust | Manual memory management rules. |
| **RAK-09** | SR-01: Practical | BK-01: RBE Tutorials | Hands-on code snippets. |
| **RAK-10** | SR-01: Domains | BK-01: WebAssembly | Compiling Rust to WASM. |

---

## 📜 Log Sinkronisasi (Spec-Log)

| Edisi Rust | Tanggal Audit | Perubahan Arsitektur | Status |
| :--- | :--- | :--- | :--- |
| **2024 Edition** | 2026-03-19 | Inisialisasi 10-Rack (Source-Driven Rust Bookshelf). | ✅ Synced |
