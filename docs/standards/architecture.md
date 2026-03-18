# Arsitektur & Hierarki Struktur (Rust Edition)

Proyek **Rust Knowledge Base** disusun dengan analogi **The Rust Bookshelf (Rak Buku Rust)** untuk mentransformasi dokumentasi teknis (seperti *The Rust Programming Language*, *The Reference*, & *Rust by Example*) menjadi unit pelajaran yang sistematis.

## Analogi Struktur

Berikut adalah pemetaannya ke dalam direktori bertingkat:

| Tingkatan | Analogi | Contoh Direktori | Keterangan |
| :--- | :--- | :--- | :--- |
| **Level 1** | **Ruang Baca (Library)** | `/` (root) | Seluruh sistem proyek (Rust Engine). |
| **Level 2** | **Rak (Shelf)** | `RAK-01-fundamentals/` | Pengelompokan besar domain ilmu (10 RAK). |
| **Level 3** | **Sub-Rak (Sub-shelf)** | `SR-01_Foundations/` | Grup materi berbasis area spesifik. |
| **Level 4** | **Buku (Book)** | `BK-01_Ownership/` | Koleksi bab yang membentuk satu topik besar. |
| **Level 5** | **Bab (Chapter)** | `CH-01_Overview/` | Unit terkecil wajib (Folder Bab). |

---

## Aturan Pewajiban `README.md`

Guna memudahkan navigasi, setiap tingkatan direktori **WAJIB** memiliki file `README.md`:

- **Root (`/README.md`)**: Visi keseluruhan (The Fearless Systems Architect 🦀).
- **Rak (`RAK-XX/README.md`)**: Tujuan dan cakupan Rak tersebut.
- **Buku (`BK-XX/README.md`)**: Sinopsis dan orientasi filosofis materi.
- **Bab (`CH-XX/README.md`)**: Materi inti (PPM Stage 1).

---

## Karakteristik & Autentisitas (Branding)

Untuk menjaga "nyawa" dan keunikan bahasa Rust, setiap konten wajib mengikuti pedoman berikut:

- **Analogi Utama**: **Penjaga Tanpa Rasa Takut (The Fearless Guardian)**.
- **Tone Suara**: **Disiplin, Tegas, dan Berdaya**. Rust tidak berkompromi soal keamanan; tulislah narasi yang memberikan rasa percaya diri kepada pembaca melalui aturan yang ketat.
- **Filosofi Penulisan**: Fokus pada *Ownership, Safety,* dan *Zero-Cost Abstractions*. Jelaskan mengapa "berantem" dengan compiler di awal adalah sebuah investasi keamanan jangka panjang.
- **Visual**: Gunakan estetika yang "industrial" dan kokoh, dengan diagram yang memvisualisasikan memori dan *borrow checker*.

---

## Kriteria "Gold Standard" (100% Complete)

Sebuah unit dianggap **Completed** jika memenuhi 4 pilar kualitas:

> [!IMPORTANT]
> **Pilar 1: Documentation-Sync Accurate**  
> Konten diverifikasi terhadap dokumentasi resmi Rust (Target di kolom Spec-Sync).
>
> **Pilar 2: Functional Examples**  
> Minimal 1 contoh kode (`.rs`) di folder `examples/`.  
> *Catatan*: Pastikan kode mengikuti *idiomatic Rust*.
>
> **Pilar 3: Mental Model Visual**  
> Minimal 1 diagram (Mermaid/SVG) di folder `assets/`.
>
> **Pilar 4: Narrative Excellence**  
> Penjelasan menggunakan standar PPM V4: Manusiawi, ada analogi (misal: "Borrow Checker as a Librarian"), dan menggunakan **Bahasa Arsitek**.
