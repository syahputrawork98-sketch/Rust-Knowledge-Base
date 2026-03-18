# Alur Kerja Penulisan (PPM) V4 - Rust Edition

Prosedur Penulisan Materi (PPM) memastikan setiap Bab di Rust Knowledge Base memiliki kualitas yang setara dengan repositori "Gold Standard" lainnya.

## Tahapan PPM (Prosedur Penulisan Materi) V4

Setiap pengerjaan Bab wajib mengikuti 4 Tahapan berikut secara disiplin:

### 1. Tahap 1: Definisi & Konsep ("Apa itu?")
- **Deskripsi**: Menjelaskan definisi materi atau fitur yang dibahas secara gamblang.
- **Landasan**: Memberikan pemahaman awal yang kuat sebelum masuk ke detail teknis.
- **Terminologi Teknis**: Mencantumkan dan menjelaskan istilah-istilah kunci Rust (Senior Terms).

### 2. Tahap 2: Rasionalitas ("Why & How?")
- **Kenapa**: Menjelaskan alasan fitur tersebut diciptakan dan masalah apa yang ia selesaikan.
- **Bagaimana**: Menjelaskan mekanisme kerja fitur tersebut (under the hood).
- **Analogi**: Gunakan analogi yang relevan dengan filosofi "Safety & Performance".

### 3. Tahap 3: Implementasi (Examples)
- **Komprehensif**: Kode di folder `examples/` harus bersifat **lengkap** dan mampu menggambarkan seluruh materi.
- **Validasi**: Pastikan skrip `.rs` valid, dapat dikompilasi, dan menunjukkan efek yang diharapkan.

### 4. Tahap 4: Visualisasi (Assets)
- **Sequence**: Diagram (Mermaid/SVG) dibuat **setelah** seluruh narasi (Tahap 1 & 2) selesai.
- **Proses**: Visualisasikan alur kepemilikan data, borrowing, atau lifetime ke dalam folder `assets/`.

---
*Target Akhir: Mencapai [Gold Standard](./architecture.md#kriteria-gold-standard-100-complete).*
