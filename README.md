# Rust Knowledge Base

Rust Knowledge Base adalah sebuah perpustakaan digital interaktif yang dirancang untuk membongkar misteri, mekanisme internal, dan arsitektur inti dari bahasa Rust.

Repositori ini secara spesifik bertindak sebagai **"The Brain"** dalam *Master Plan: Polyglot Senior Architect*. Fokus materi di sini murni pada **Core Language Rust** (seperti *Ownership, Borrowing, Lifetimes, Memory Safety*), dan **bukan** tempat untuk membahas Framework UI (seperti Yew) atau Web Frameworks (seperti Axum) yang memiliki ekosistem sendiri.

## Struktur Perpustakaan (5-Level Hierarchy)

Sesuai standar **PPM (Perpustakaan Pribadi Modular)**, setiap materi mengikuti hierarki:
**Rak -> Sub-Rak -> Buku -> Bab -> Section.**

```mermaid
graph TD
    Root["Rust Knowledge Base"]
    
    RAK01["RAK-01-fundamentals<br/>(Core Concepts)"]
    RAK02["RAK-02-advanced<br/>(Deeper Internals)"]
    RAK03["RAK-03-types<br/>(Traits & Generics)"]
    
    Root --> RAK01
    Root --> RAK02
    Root --> RAK03
    
    RAK01 --> SR01["SR-01_MemorySafety"]
    RAK01 --> SR02["SR-02_Concurrency"]
    
    style Root fill:#f9f,stroke:#333,stroke-width:4px
    style RAK01 fill:#bbf,stroke:#333
    style RAK02 fill:#bfb,stroke:#333
    style RAK03 fill:#fbb,stroke:#333
```

---

## Roadmap & Status Pengembangan

| Rak | Deskripsi | Status |
| :--- | :--- | :--- |
| `RAK-01-fundamentals/` | Ownership, Borrowing, Lifetimes | 1% | [/] In Progress |
| `RAK-02-advanced/` | Unsafe Rust, FFI, Macros | *Planned* |
| `RAK-03-types/` | Traits, Generics, Zero-cost abstractions | *Planned* |

## Visi & Tujuan

Repository ini bukan sekadar tempat menaruh *snippet* kode, melainkan sebuah laboratorium mental untuk memahami "The Soul of Rust".

---
*Dokumentasi Lengkap & Roadmap: [catatan](file:///i:/Workspace/Workspace-Syahputrawork/catatan/01-Language-Hubs/Rust-Knowledge-Base.md)*