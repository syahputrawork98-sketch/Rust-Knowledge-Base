# Authoring Guide

Panduan ini menjelaskan bagaimana materi Rust ditulis, kapan sebuah unit dianggap selesai, dan kualitas minimum yang harus dijaga untuk narasi, contoh kode, dan visual.

## 1. PPM V4

Setiap unit `CH` atau `SEC` normal mengikuti 5 tahap:

### Tahap 1. Source Alignment dan Judul
- judul unit;
- tautan ke sumber primer yang relevan;
- framing singkat atau analogi bila membantu.

### Tahap 2. Konsep dan Rasionalitas
- definisi formal;
- alasan konsep itu penting;
- model mental atau analogi;
- terminologi teknis yang tepat.

### Tahap 3. Visual Logic
- Mermaid inline di dalam `README.md` jika visual memang membantu pemahaman.

### Tahap 4. Under-the-hood
- penjelasan mekanisme internal yang relevan;
- jelaskan apakah pembahasan ada di level bahasa, standard library, compiler, runtime, atau domain.

### Tahap 5. Lab Praktis
- rujukan ke `examples/`;
- contoh sebaiknya runnable, testable, atau sengaja menunjukkan perilaku compile-time yang penting;
- file bernomor urut.

## 2. Quality Bar

Sebuah unit `CH` atau `SEC` normal dianggap kuat jika memiliki:
1. source primer yang jelas;
2. definisi formal dan analogi;
3. penjelasan rasionalitas;
4. visual logic bila relevan;
5. penjelasan under-the-hood;
6. lab praktis bila relevan;
7. pitfalls, batasan, atau miskonsepsi umum.

Unit boleh ditandai `Complete` jika:
- quality bar tersebut terpenuhi secara memadai; atau
- unit itu sah sebagai `Nil Content`.

## 3. Nil Content

Jika unit murni historis, filosofis, atau naratif:
- cukup `README.md`;
- jangan buat `examples/` kosong;
- jangan buat `assets/` kosong;
- tulis penafian eksplisit.

Contoh penafian:

`Unit ini tidak membutuhkan Lab Praktis atau aset visual tambahan karena bersifat penjelasan sejarah atau konsep naratif.`

## 4. Standar Narasi

- akurat terhadap sumber resmi;
- teknis, tetapi tetap bisa dipahami;
- analogi dipakai untuk menjelaskan, bukan mengganti definisi;
- jangan membuat istilah baru jika istilah resmi Rust sudah ada;
- bedakan dengan jujur antara fakta spesifikasi, perilaku compiler, dan intuisi pembelajaran.

## 5. Standar Kode

- contoh kode harus self-contained sejauh memungkinkan;
- contoh boleh berupa compile-pass atau compile-fail jika itu memang inti pelajarannya;
- gunakan idiom Rust yang benar;
- komentar dipakai seperlunya;
- nama file di `examples/` sebaiknya seperti:
  - `01_basic_move.rs`
  - `02_borrow_rules.rs`
  - `03_compile_error.rs`

## 6. Standar Visual

- Mermaid inline adalah default;
- `assets/` hanya dipakai jika visual eksternal benar-benar dibutuhkan;
- visual harus membantu reasoning, bukan dekorasi;
- diagram yang baik menonjolkan alur ownership, borrowing, state transition, atau boundary sistem.

## 7. Prioritas Sumber

Gunakan prioritas sumber ini:
1. Rust Reference
2. The Rust Programming Language
3. Standard library docs
4. Rustonomicon
5. rustc dev guide
6. RFC atau dokumentasi resmi domain spesifik

Catatan:
- untuk topik `RAK-06`, prioritaskan sumber primer paling ketat;
- untuk topik `RAK-02`, The Book dan standard library docs biasanya cukup sebagai landasan awal;
- jangan mengandalkan blog pihak ketiga jika sumber resmi sudah memadai.

## 8. Kontribusi dan Refactor

Saat menulis atau merefaktor materi:
1. tentukan dulu lokasi unit dalam hirarki repo;
2. baca `docs/standards/README.md`;
3. baca `docs/repository-plan/README.md` jika scope raknya belum jelas;
4. ikuti PPM V4;
5. jangan meninggalkan folder kosong;
6. perbarui status hanya jika ada bukti yang cukup.

## 9. Template Minimum

Repo ini sengaja tidak bergantung pada banyak file template. Gunakan pola minimum berikut bila perlu.

### Template `CH` atau `SEC`

```md
# [Judul]

## Source Link
- [Sumber primer](#)

## Definisi Singkat

## Kenapa Ini Penting

## Model Mental

## Visual Logic

## Under-the-hood

## Lab Praktis

## Pitfalls
```

### Template Status `BK`

```md
| Bab | Judul | Status | Lab | Visual | Source |
| :--- | :--- | :--- | :---: | :---: | :--- |
| CH-01 | Overview | Draft | No | No | The Book |
```

Jika template terasa terlalu besar untuk unit tertentu, sederhanakan. Template ada untuk membantu konsistensi, bukan menambah beban.
