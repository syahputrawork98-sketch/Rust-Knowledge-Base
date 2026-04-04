# Repository Guide

Panduan ini adalah pintu masuk utama untuk memahami aturan kerja `Rust Knowledge Base`. Tujuannya adalah merangkum struktur repo, istilah, status, dan boundary dasar dalam satu dokumen yang ringkas.

## 1. Cara Baca Dokumen

Urutan baca yang direkomendasikan:
1. file ini, `docs/standards/README.md`
2. [authoring.md](./authoring.md)
3. [../repository-plan/README.md](../repository-plan/README.md)

Jika ada konflik aturan, gunakan urutan ini:
1. `docs/standards/README.md`
2. `docs/standards/authoring.md`
3. `docs/repository-plan/README.md`
4. `.cursorrules`
5. `README.md`
6. `status.md`

## 2. Struktur Repo

Repositori ini memakai hierarki:

1. `Root`
2. `RAK`
3. `SR`
4. `BK`
5. `CH`
6. `SEC`

Pengecualian:
- `RAK-01` boleh melewati `SR` jika unitnya murni naratif dan pengantar.

## 3. Fungsi Tiap Level

| Level | Prefix | Fungsi |
| :--- | :--- | :--- |
| Root | `/` | Hub utama repositori |
| RAK | `RAK-` | Domain besar pembelajaran |
| SR | `SR-` | Track spesifik di dalam rak |
| BK | `BK-` | Kumpulan chapter yang membentuk modul |
| CH | `CH-` | Unit materi utama |
| SEC | `SEC-` | Detail terdalam bila benar-benar dibutuhkan |

## 4. Naming Convention

Gunakan format berikut:

| Level | Format | Contoh |
| :--- | :--- | :--- |
| RAK | `RAK-XX-slug` | `RAK-04-core-mechanics` |
| SR | `SR-XX-slug` | `SR-01-ownership-basics` |
| BK | `BK-XX_Slug` | `BK-01_Ownership` |
| CH | `CH-XX_Slug` | `CH-01_BorrowingRules` |
| SEC | `SEC-XX_Slug` | `SEC-01_AliasingModel` |

Aturan tambahan:
- `RAK` dan `SR` memakai tanda hubung `-`;
- `BK`, `CH`, dan `SEC` memakai garis bawah `_` setelah nomor;
- file di `examples/` memakai prefix numerik berurutan, misalnya `01_basic_move.rs`.

## 5. Struktur Minimum Folder

### Untuk `RAK`, `SR`, dan `BK`
- wajib punya `README.md` sebagai navigasi atau ringkasan.

### Untuk `CH` dan `SEC`
- wajib punya `README.md`;
- boleh punya `examples/`;
- boleh punya `assets/`.

Prinsip penting:
- `examples/` dan `assets/` hanya dibuat jika memang dipakai;
- jangan meninggalkan folder kosong;
- Mermaid inline adalah default, jadi aset visual tambahan bukan kewajiban.

## 6. Terminologi yang Dipakai

Gunakan istilah Rust yang resmi dan presisi, misalnya:
- `ownership`
- `borrowing`
- `lifetime`
- `trait`
- `monomorphization`
- `interior mutability`
- `unsafe`

Hindari istilah yang terlalu marketing atau terlalu umum jika istilah resmi Rust sudah ada.

## 7. Aturan Status

Gunakan status operasional berikut saja:
- `Draft`
- `Partial`
- `Complete`

Maknanya:
- `Draft`: struktur narasi inti sudah ada, tetapi pembuktian atau pendalaman belum lengkap;
- `Partial`: materi utama sudah kuat, tetapi masih ada bagian seperti lab, visual, atau audit yang belum selesai;
- `Complete`: unit sudah memenuhi quality bar atau sah sebagai `Nil Content`.

Status dasar dicatat di level `CH` atau `SEC`, lalu naik ke `BK`, `SR`, `RAK`, dan `status.md`.

Prinsipnya:
- status tidak boleh lebih optimistis dari bukti;
- jika audit belum selesai, pakai status yang konservatif.

## 8. Boundary Antar Rak

Untuk menghindari tumpang tindih:

| Rak | POV |
| :--- | :--- |
| `RAK-02` | Pemakaian Rust yang benar dalam praktik sehari-hari |
| `RAK-04` | Reasoning model dan semantik inti Rust |
| `RAK-05` | Toolchain, standard library, dan lingkungan kerja |
| `RAK-06` | Unsafe boundary, memory layout, FFI, dan detail rendah |
| `RAK-07` | Adaptasi prinsip Rust untuk domain spesifik |

## 9. Dokumen yang Benar-Benar Penting

Dokumen inti repo ini adalah:
- `README.md`
- `docs/standards/README.md`
- `docs/standards/authoring.md`
- `docs/repository-plan/README.md`
- `.cursorrules`
- `status.md`

Untuk layer standar sendiri, source of truth sengaja dipersempit menjadi:
- `docs/standards/README.md`
- `docs/standards/authoring.md`

Jika sebuah aturan belum layak menjadi bagian dari dua file ini, biasanya aturan itu belum cukup penting untuk hidup sebagai dokumen terpisah.
