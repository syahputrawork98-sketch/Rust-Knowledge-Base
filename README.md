# Rust Knowledge Base

> **"Empowering Everyone to Build Reliable and Efficient Software."**

## 🏛️ Arsitektur 7-Rak (Universal Standard)
Repositori ini menggunakan **7-Rack Universal Architecture** dengan prinsip **Digital Mirroring** untuk memisahkan antara fondasi penggunaan dengan dekonstruksi arsitektur mesin.

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#DEA584', 'primaryTextColor': '#000'}}}%%
graph TD
    Root["Rust Knowledge Base"]
    
    RAK01["RAK-01-anatomy<br/>(The Landscape)"]
    RAK02["RAK-02-foundation<br/>(The Standard Book)"]
    RAK03["RAK-03-evolution<br/>(History & Future)"]
    RAK04["RAK-04-core-mechanics<br/>(The Internal Logic)"]
    RAK05["RAK-05-ecosystem<br/>(The Environment)"]
    RAK06["RAK-06-the-underworld<br/>(The Machine Room)"]
    RAK07["RAK-07-specialization<br/>(The Cutting Edge)"]
    
    Root --> RAK01 & RAK02 & RAK03 & RAK04 & RAK05 & RAK06 & RAK07
    
    style Root fill:#DEA584,stroke:#333,stroke-width:4px,color:#000
    style RAK01 fill:#fff,stroke:#333
    style RAK02 fill:#fff,stroke:#333
    style RAK03 fill:#fff,stroke:#333
    style RAK04 fill:#ddd,stroke:#333
    style RAK05 fill:#fff,stroke:#333
    style RAK06 fill:#ddd,stroke:#333
    style RAK07 fill:#fff,stroke:#333
```

---

## 🗄️ Struktur Perpustakaan

### 1. [RAK-01-anatomy](./RAK-01-anatomy/)
Filosofi desain memori aman tanpa GC, sejarah, dan batasan *The Triangle of Trade-offs*.

### 2. [RAK-02-foundation](./RAK-02-foundation/)
Sintaks dan tipe data dasar yang bersumber langsung dari *The Rust Programming Language*.

### 3. [RAK-03-evolution](./RAK-03-evolution/)
Evolusi Rust Editions (2015, 2018, 2021) dan target perkembangan kompilator masa depan.

### 4. [RAK-04-core-mechanics](./RAK-04-core-mechanics/)
Mekanika Paling Mendalam: *Ownership*, *Borrow Checking*, *Lifetimes*, dan *Traits*.

### 5. [RAK-05-ecosystem](./RAK-05-ecosystem/)
Ekosistem Rust: perbedaan `std` vs `core`, Cargo Toolchain, dan Crates.io.

### 6. [RAK-06-the-underworld](./RAK-06-the-underworld/)
Deep dive ke *Unsafe Rust*, *FFI*, *Raw Pointers*, dan *Memory Layout*.

### 7. [RAK-07-specialization](./RAK-07-specialization/)
Implementasi domain spesifik: *WebAssembly (Wasm)*, *Async Rust*, dan *Embedded*.

---

## 📏 Standar Kualitas (Gold Standard)
Setiap materi mengikuti prinsip **Digital Mirroring** dan standar **PPM V4**:
1. **Source-Synced**: Akurasi 1:1 terhadap dokumentasi resmi/spesifikasi.
2. **Experimental Lab**: Kode pembuktian fungsional di folder `examples/`.
3. **Mental Model Visual**: Diagram Mermaid inline di `README.md`.
4. **Narrative Excellence**: Penjelasan mendalam dengan analogi sistem.

*Dokumentasi Lengkap Standar: [docs/standards/architecture.md](./docs/standards/architecture.md)*

---
*Status Pengembangan: [status.md](./status.md)*