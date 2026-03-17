# Standar: Protokol Pembaruan Status (Bottom-Up Aggregation)

Dokumen ini mendefinisikan prosedur standar untuk memperbarui status kemajuan di seluruh hierarki **Rust Knowledge Base**. Tujuannya adalah untuk memastikan data yang akurat, konsisten, dan mudah dilacak dari level terendah hingga level tertinggi.

## 1. Hierarki Status

| Level | File Lokasi | Fokus Utama |
| :--- | :--- | :--- |
| **Bab (CH)** | `CH-XX/README.md` | Narasi, Analogi, Istilah, SVG, Example. |
| **Buku (BK)** | `BK-XX/docs/status.md` | Granular: Status Bab, **Section Count**, SVG/Example Count, Source Sync. |
| **Sub-Rak (SR)** | `SR-XX/docs/status.md` | Agregasi: Daftar Buku dalam Sub-Rak. |
| **Rak (RAK)** | `RAK-XX/docs/status.md` | Agregasi: Daftar Sub-Rak, Persentase Penyelesaian Sub-Rak. |
| **Root (HUB)** | `docs/status.md` | Master: Status Keseluruhan Hub (High-Level Roadmap). |

---

## 2. Prosedur Pembaruan (Bottom-Up)

Setiap kali ada perubahan material (misal: Bab selesai ditulis), pembaruan status HARUS dilakukan secara berjenjang:

### Langkah 1: Audit & Update Level Buku (`BK`)
- **Review Materi**: Periksa setiap `README.md` pada bab terkait (Narasi + Analogi + Istilah Resmi).
- **Validasi Aset**: 
    - Hitung folder `examples/` (Example Count).
    - Hitung aset grafis (SVG/Mermaid Count).
- **Update File**: Perbarui baris tabel di `BK-XX/docs/status.md`.
- **Cek Up-to-date**: Pastikan materi masih sesuai dengan versi terbaru *The Rust Book*.

---

## 3. Kriteria Penyelesaian (Definition of Done)

Sebuah Bab/Buku hanya boleh ditandai **100% [x]** jika memenuhi kriteria "Gold Standard":
1.  **Narasi & Analogi**: Sudah lengkap dan mematuhi "Kaidah Istilah".
2.  **Asset**: Minimal memiliki 1 Visualisasi (SVG/Mermaid).
3.  **Code**: Minimal memiliki 1 Contoh Kode praktis.
4.  **Recency**: Status "Source Sync" adalah "Up-to-date".

Jika salah satu kriteria di atas belum terpenuhi, gunakan status:
- `Draft`: Narasi awal tersedia, aset/kode belum ada.
- `Ongoing`: Sedang dalam pengerjaan (Narasi/Kode).
- `Outdated`: Materi perlu diperbarui sesuai sumber resmi terbaru.

### Langkah 2: Agregasi ke Level Sub-Rak (`SR`)
- Hitung rata-rata persentase dari seluruh Buku (`BK`) di bawahnya.
- Update `SR-XX/docs/status.md`.

### Langkah 3: Agregasi ke Level Rak (`RAK`)
- Hitung rata-rata persentase dari seluruh Sub-Rak (`SR`) di bawahnya.
- Update `RAK-XX/docs/status.md`.

### Langkah 4: Sinkronisasi Master Status (HUB)
- Perbarui `docs/status.md` untuk mencerminkan status terbaru dari setiap Rak.

---
> [!IMPORTANT]
> **Kaidah 100%**: Sebuah level hanya dianggap 100% jika seluruh level di bawahnya telah diverifikasi 100% (Narasi lengkap, Kode valid, Diagram Mermaid tersedia).

---
*Back to [Structure Guide](../structure-guide.md)*
