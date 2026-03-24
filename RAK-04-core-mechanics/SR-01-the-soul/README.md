# SR-01-the-soul (Memory Passport)

> **"Pusat Mekanika Memori: Ownership, Borrowing, dan Lifetimes."**

Sub-Rak ini membedah "Jiwa" dari Rust. Inilah alasan mengapa Rust bisa menjamin keamanan memori tanpa menggunakan Garbage Collector.

## 📚 Rencana Pengembangan (Books & Chapters)

### BK-01_Ownership_Model
Aturan emas kepemilikan data.
- **CH-01_Move_Semantics**: Bagaimana Rust memindahkan data antar variabel.
- **CH-02_Copy_Types**: Data primitif yang bisa diduplikasi secara otomatis di stack.

### BK-02_Borrowing_Rules
Meminjamkan data dengan referensi.
- **CH-01_Immutable_References**: Berbagi data secara read-only.
- **CH-02_Mutable_References**: Aturan eksklusifitas tulis.
- **CH-03_Aliasing_Rules**: Mencegah data races di waktu kompilasi.

### BK-03_Lifetimes
Menjamin validitas referensi.
- **CH-01_Static_Lifetimes**: Referensi yang hidup selama program berjalan.
- **CH-02_Generic_Lifetimes**: Memberi label pada hubungan antar referensi dalam fungsi/struct.

---
## 📊 Status Sub-Rak
Status: ⚪ **Skeleton Ready**

---
*Kembali ke [RAK-04-core-mechanics](../README.md)*
