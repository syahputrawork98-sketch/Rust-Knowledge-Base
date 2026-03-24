# SR-01-unsafe-contracts (The Dark Arts)

> **"Menembus Batas Keamanan dengan Tanggung Jawab Penuh."**

Sub-Rak ini membedah *Unsafe Rust*. Kita mempelajari kapan harus menggunakan blok `unsafe`, kontrak keamanan apa yang harus dipenuhi, dan bagaimana mengisolasi kode berbahaya di balik abstraksi aman.

## 📚 Rencana Pengembangan (Books & Chapters)

### BK-01_Unsafe_Basics
Fondasi penggunaan keyword `unsafe`.
- **CH-01_Dereferencing_Raw_Pointers**: Cara menggunakan pointer mentah (`*const T`, `*mut T`).
- **CH-02_Unsafe_Functions**: Memanggil fungsi yang ditandai sebagai `unsafe`.

### BK-02_Memory_Safety_Manual
Prinsip-prinsip agar kode unsafe tetap aman.
- **CH-01_Undefined_Behavior**: Mengenali apa yang dilarang keras di Rust.
- **CH-02_Safety_Invariants**: Menjaga integritas data secara manual.

---
## 📊 Status Sub-Rak
Status: ⚪ **Skeleton Ready**

---
*Kembali ke [RAK-06-the-underworld](../README.md)*
