# Rak 12: High-Performance Wasm

Membawa performa bahasa sistem (Rust) ke dalam ekosistem Web via WebAssembly.

## Struktur Sub-Rak
- **[SR-01-WASM-BRIDGE](./SR-01-WASM-BRIDGE/README.md)**: Komunikasi data antara JS/TS dan Rust Wasm.
- **[SR-02-GRAPHICS-ENGINE](./SR-02-GRAPHICS-ENGINE/README.md)**: Mengolah WebGL/WebGPU dengan Rust.

## Why Rust for Wasm?
Rust adalah bahasa yang paling dicintai untuk WebAssembly karena:
1. **Tooling Terdepan**: Ecosystem `wasm-pack` adalah yang terbaik di kelasnya.
2. **Predictable Performance**: Tidak ada jeda *Garbage Collection* di sisi klien.
3. **Shared Logic**: Logika backend Rust bisa dipakai di frontend secara utuh.

---

## 🌐 Architectural Nexus (Jembatan Master Plan)
Materi di Rak ini terhubung langsung dengan komponen lain dalam ekosistem [Master Plan Senior](../../catatan/Master-Plan-Senior.md):
- **Wasm Frontend**: Lihat [Rust WASM UI Lab](../../catatan/03-Digital-UI-Hubs/Rust-WASM-UI-Lab.md) untuk implementasi UI.
- **Client Runtime**: Hubungan performa dengan [Browser Runtime Knowledge Base](../../catatan/02-Execution-Hubs/Browser-Runtime-Knowledge-Base.md).

---
*Kembali ke [Documentation Hub](../docs/README.md)*
