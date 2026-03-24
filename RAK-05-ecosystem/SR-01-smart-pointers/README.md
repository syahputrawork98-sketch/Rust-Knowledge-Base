# SR-01-smart-pointers (The Envelopes)

> **"Mengelola Alokasi Memori dengan Pintar: Box, Rc, dan Arc."**

Sub-Rak ini membedah berbagai tipe data pointer di Rust yang memberikan kemampuan manajemen memori otomatis dan aman melalui mekanisme RAII.

## 📚 Rencana Pengembangan (Books & Chapters)

### BK-01_Heap_Allocation
Memahami pemindahan data ke heap.
- **CH-01_Box_T**: Penggunaan `Box<T>` untuk alokasi heap sederhana.
- **CH-02_Recursive_Types**: Menyelesaikan masalah tipe rekursif dengan indirection.

### BK-02_Reference_Counting
Berbagi kepemilikan data antar bagian program.
- **CH-01_Rc_T**: Multiple ownership untuk single-threaded.
- **CH-02_Arc_T**: Thread-safe reference counting untuk multi-threaded.

### BK-03_Interior_Mutability
Mengubah data di balik referensi immutable.
- **CH-01_RefCell_T**: Mekanisme interior mutability di runtime.
- **CH-02_Mutex_RwLock**: Interior mutability yang aman secara thread.

---
## 📊 Status Sub-Rak
Status: ⚪ **Skeleton Ready**

---
*Kembali ke [RAK-05-ecosystem](../README.md)*
