# SR-02-interop-ffi (The Bridge)

> **"Berbicara dengan Dunia Luar melalui FFI."**

Sub-Rak ini membedah mekanisme interoperabilitas Rust dengan bahasa lain (terutama C) melalui *Foreign Function Interface*.

## 📚 Rencana Pengembangan (Books & Chapters)

### BK-01_External_Functions
Memanggil fungsi dari library eksternal.
- **CH-01_Linking_Libraries**: Cara memberi tahu Cargo untuk melakukan linking.
- **CH-02_Extern_Blocks**: Mendefinisikan signature fungsi luar.

### BK-02_Data_Layout
Menjamin struktur data Rust kompatibel dengan C.
- **CH-01_Repr_C**: Menggunakan atribut `#[repr(C)]` untuk kestabilan layout.
- **CH-02_Opaque_Types**: Menangani pointer ke tipe data yang tidak diketahui strukturnya.

---
## 📊 Status Sub-Rak
Status: ⚪ **Skeleton Ready**

---
*Kembali ke [RAK-06-the-underworld](../README.md)*
