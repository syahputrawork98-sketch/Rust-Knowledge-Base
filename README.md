# Rust Knowledge Base

> **"Safety, Speed, and Systems: The Path to Rust Architecture."**

## Latar Belakang & Visi
Rust adalah bahasa masa depan untuk pengembangan sistem yang aman dan performan. Repositori ini membedah "Rust Bookshelf" secara terstruktur, dari kepemilikan memori (Ownership) hingga pemrograman tingkat rendah (Unsafe Rust).

## Struktur Perpustakaan (10-Rack Architecture)

```mermaid
graph TD
    Root["Rust Knowledge Base"]
    
    RAK01["RAK-01-get-started<br/>(The Entry)"]
    RAK02["RAK-02-the-book<br/>(The Soul)"]
    RAK03["RAK-03-reference<br/>(The Rules)"]
    RAK04["RAK-04-std-library<br/>(The API)"]
    RAK05["RAK-05-cargo-book<br/>(The Eco)"]
    RAK06["RAK-06-toolchain<br/>(The Workbench)"]
    RAK07["RAK-07-edition-guide<br/>(The Timeline)"]
    RAK08["RAK-08-advanced-rust<br/>(The Underworld)"]
    RAK09["RAK-09-rust-by-example<br/>(The Practical)"]
    RAK10["RAK-10-specialized<br/>(The Domains)"]
    
    Root --> RAK01 & RAK02 & RAK03 & RAK04 & RAK05 & RAK06 & RAK07 & RAK08 & RAK09 & RAK10
    
    style Root fill:#000,stroke:#fff,stroke-width:4px,color:#fff
    style RAK01 fill:#fff,stroke:#333
    style RAK02 fill:#fff,stroke:#333
```

## Roadmap & Status Pengembangan

| Rak | Deskripsi | Status |
| :--- | :--- | :--- |
| `RAK-01-get-started/` | Setup & Rustup | *Planned* |
| `RAK-02-the-book/` | Core Ownership Logic | *Planned* |
| `RAK-03-reference/` | Language Specification | *Planned* |
| `RAK-04-std-library/` | API Docs & Collections | *Planned* |
| `RAK-05-cargo-book/` | Dependency Management | *Planned* |
| `RAK-06-toolchain/` | rustc, rustdoc, clippy | *Planned* |
| `RAK-07-edition-guide/` | 2015 to 2024 Transition | *Planned* |
| `RAK-08-advanced-rust/` | Unsafe, Nomicon, Macros | *Planned* |
| `RAK-09-rust-by-example/` | Tutorial Snippets | *Planned* |
| `RAK-10-specialized/` | WASM, Embedded, CLI | *Planned* |

---
*Dokumentasi Lengkap: [docs/README.md](./docs/README.md)*