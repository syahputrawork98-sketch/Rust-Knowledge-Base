# Rust Knowledge Base Blueprint

Dokumen ini memetakan arah besar `Rust-Knowledge-Base`. Fokusnya adalah strategi isi, boundary antar-rak, dan DNA kualitas yang harus dijaga saat repo ini bertumbuh.

Aturan teknis mikro tidak hidup di sini. Blueprint ini menjawab:
- repo ini mau dibentuk seperti apa;
- apa yang diwarisi dari repo terbaik yang sudah ada;
- bagaimana materi Rust dibagi agar tidak tumpang tindih.

## 1. Arah Desain

Blueprint ini sengaja memakai pendekatan hybrid:

- warisi **governance yang ringkas** dari `Golang-Knowledge-Base`;
- warisi **kedalaman teknis dan mental model** dari `JavaScript-Knowledge-Base`;
- hindari kelemahan keduanya dengan menjaga repo tetap **self-contained**, jelas, dan tidak terlalu berat secara operasional.

Artinya, repo Rust ini harus:
- mudah dipahami AI dan kontributor baru;
- tetap tajam saat membahas compiler, memory model, dan unsafe boundary;
- tidak bergantung pada blueprint eksternal di luar repo;
- tidak menambah dokumen atau ritual kerja yang tidak benar-benar membantu.

## 2. Prinsip Hybrid

### Yang diwarisi dari Go
- source of truth harus jelas dan pendek;
- jumlah dokumen inti harus sedikit tapi kuat;
- istilah repo harus stabil;
- status harus jujur dan konservatif;
- struktur harus lebih penting daripada improvisasi folder.

### Yang diwarisi dari JavaScript
- setiap materi harus punya sudut pandang teknis yang tajam;
- source primer wajib diprioritaskan untuk topik low-level;
- penjelasan tidak cukup berhenti di definisi, tetapi harus sampai ke mental model;
- topik sulit perlu dibedah dengan under-the-hood reasoning, pitfalls, dan boundary yang jelas.

### Yang tidak ikut diwarisi
- mode interaksi yang terlalu banyak;
- permission gate yang terlalu sering;
- artifact session seperti `task.md` atau `analysis_report.md` sebagai kewajiban repo;
- branding yang lebih dominan daripada keterbacaan aturan;
- ketergantungan ke path atau dokumen eksternal di luar repositori ini.

## 3. DNA Teknis Rust

Semua materi Rust di repo ini pada akhirnya harus kembali ke enam lensa utama:

1. **Ownership and Borrowing**
   Menjelaskan siapa pemilik data, siapa boleh meminjam, dan kapan akses itu berakhir.
2. **Type System and Traits**
   Menjelaskan bagaimana Rust membangun safety, generic power, dan static reasoning.
3. **Memory Layout and Zero-Cost Abstractions**
   Menjelaskan kapan abstraksi hilang saat kompilasi, dan kapan biaya nyata masih ada.
4. **Concurrency and Aliasing Safety**
   Menjelaskan hubungan `Send`, `Sync`, interior mutability, dan race prevention.
5. **Unsafe Boundary**
   Menjelaskan peralihan dari safe Rust ke area yang menuntut kontrak manual.
6. **Compiler Reasoning**
   Menjelaskan bagaimana `rustc` berpikir, dari semantic checking sampai code generation bila relevan.

Jika sebuah unit tidak jelas jatuh ke lensa mana, biasanya scope unit tersebut masih kabur.

## 4. Arsitektur Repo

Repositori ini memakai struktur:

1. `Root`
2. `RAK`
3. `SR`
4. `BK`
5. `CH`
6. `SEC`

Pengecualian:
- `RAK-01` boleh melewati `SR` jika unitnya murni naratif dan pengantar.

Prinsip pertumbuhannya:
- tambah kedalaman hanya jika memang dibutuhkan;
- `SEC` dipakai untuk detail yang benar-benar memecah fokus dari `CH`;
- jangan membuat folder karena "terasa rapi" jika isi belum menuntutnya.

## 5. Blueprint 7 Rak

### RAK-01: Anatomy
Fokus pada asal-usul Rust, masalah yang ingin dipecahkan, filosofi desain, trade-off, dan posisi Rust di antara bahasa sistem lainnya.

Karakter:
- boleh dominan naratif;
- `Nil Content` lebih sering sah di sini;
- tujuan utamanya membangun konteks, bukan detail mekanis.

Sumber utama:
- Rust website
- The Rust Programming Language
- RFC historis yang benar-benar relevan

### RAK-02: Foundation
Fokus pada fondasi bahasa yang dipakai sehari-hari: variables, ownership dasar, references, structs, enums, pattern matching, modules, error handling, collections, dan flow control.

Karakter:
- menjawab "bagaimana menulis Rust dengan benar";
- tetap menjelaskan alasan desain, tetapi belum masuk terlalu dalam ke mekanika compiler.

Sumber utama:
- The Rust Programming Language
- Rust By Example
- Standard library docs untuk tipe dasar

### RAK-03: Evolution
Fokus pada perkembangan bahasa dan ekosistem inti: editions, RFC-driven change, stabilisasi fitur, perubahan idiom, async maturation, tooling maturity, dan arah masa depan yang sudah punya landasan resmi.

Karakter:
- menjawab "bagaimana Rust berubah dan kenapa";
- bukan tempat untuk membahas konsep dasar dari nol;
- bukan tempat untuk deep dive unsafe.

Sumber utama:
- Edition guide
- RFC
- Rust blog
- release notes resmi

### RAK-04: Core Mechanics
Fokus pada inti logika Rust: ownership lanjutan, borrowing rules, lifetimes, traits, generics, coercions, dispatch, pinning dasar, dan reasoning tentang aliasing serta validity.

Karakter:
- menjawab "kenapa aturan Rust terasa seperti ini";
- wajib kuat di mental model;
- wajib membedakan aturan bahasa, perilaku compiler, dan intuisi pengguna.

Sumber utama:
- Rust Reference
- The Rust Programming Language
- Rustonomicon bila perlu sebagai pendalaman, bukan default

### RAK-05: Ecosystem
Fokus pada lingkungan kerja utama Rust: `std`, `core`, `alloc`, Cargo, testing, documentation tooling, package ecosystem, crates, workspace patterns, dan boundary antara core language dengan toolchain.

Karakter:
- menjawab "dengan apa Rust dipakai dan dijalankan sehari-hari";
- bukan tempat utama untuk unsafe internals;
- harus jelas membedakan bahasa, standard library, dan tooling.

Sumber utama:
- Standard library docs
- Cargo Book
- Rustdoc Book
- The Book untuk konteks dasar

### RAK-06: The Underworld
Fokus pada detail paling rendah: unsafe Rust, raw pointers, memory layout, FFI, ABI caveats, atomics dasar, compiler pipeline, MIR, optimisasi, dan kontrak-kontrak yang tidak dijaga penuh oleh safe Rust.

Karakter:
- menjawab "apa yang terjadi di balik jaminan safety Rust";
- quality bar source rigor paling tinggi ada di sini;
- setiap penyederhanaan harus jujur terhadap batasnya.

Sumber utama:
- Rustonomicon
- Rust Reference
- rustc dev guide
- Unsafe Code Guidelines yang relevan

### RAK-07: Specialization
Fokus pada domain penerapan yang membutuhkan pengetahuan Rust spesifik: async, embedded, WebAssembly, high-performance services, interoperabilitas, dan domain lain yang layak diperlakukan sebagai cabang lanjut.

Karakter:
- menjawab "bagaimana prinsip Rust berubah saat masuk domain khusus";
- tidak mengulang Foundation atau Core Mechanics tanpa alasan;
- setiap domain harus tetap ditarik kembali ke DNA teknis Rust.

Sumber utama:
- Async Book
- Embedded Rust Book
- wasm-bindgen / wasm book yang relevan
- dokumentasi resmi domain masing-masing

## 6. Boundary Antar Rak

Untuk mencegah tumpang tindih:

| Rak | POV utama |
| :--- | :--- |
| `RAK-02` | Cara memakai Rust dengan benar dalam praktik sehari-hari |
| `RAK-04` | Kenapa aturan Rust bekerja seperti itu secara konseptual dan semantis |
| `RAK-05` | Library, toolchain, dan lingkungan kerja yang dipakai bersama bahasa |
| `RAK-06` | Kontrak low-level, unsafe boundary, dan detail implementasi |
| `RAK-07` | Adaptasi prinsip Rust ke domain spesifik |

Aturan sederhananya:
- jika topik utamanya adalah sintaks dan penggunaan idiomatik, arahkan ke `RAK-02`;
- jika topik utamanya adalah reasoning model, arahkan ke `RAK-04`;
- jika topik utamanya adalah library/toolchain, arahkan ke `RAK-05`;
- jika topik utamanya adalah layout, FFI, pointer, atau compiler internals, arahkan ke `RAK-06`;
- jika topik utamanya adalah penerapan dalam domain tertentu, arahkan ke `RAK-07`.

## 7. Quality Bar Materi

Setiap unit `CH` atau `SEC` normal sebaiknya lulus kualitas berikut:

1. **Source Link**
   Tautan ke sumber primer yang relevan.
2. **Formal Definition and Analogy**
   Definisi resmi dijelaskan ulang dengan model mental yang membumi.
3. **Visual Logic**
   Mermaid inline dipakai sebagai default jika visual memang membantu.
4. **Under-the-hood**
   Ada penjelasan mekanisme internal yang relevan, tidak sekadar tutorial permukaan.
5. **Practical Lab**
   Ada contoh `examples/` yang runnable atau sengaja menunjukkan compile-time behavior.
6. **Pitfalls and Misconceptions**
   Ada jebakan umum, batasan, atau miskonsepsi yang diluruskan.
7. **Boundary Awareness**
   Jelas apakah pembahasan sedang berada di level bahasa, library, compiler, runtime, atau domain.

`Nil Content` tetap sah untuk unit yang benar-benar naratif, tetapi harus diberi penafian eksplisit.

## 8. Aturan Pertumbuhan Repo

Saat repo berkembang:
- pertahankan dokumen inti tetap sedikit;
- konsolidasikan aturan, jangan biarkan standar tersebar tanpa alasan;
- prioritaskan Mermaid inline dibanding aset visual tambahan;
- jangan membuat `examples/` atau `assets/` kosong;
- jangan menaikkan status lebih tinggi dari bukti yang tersedia;
- pilih struktur yang paling mudah dipahami kontributor berikutnya.

## 9. Dokumen Inti yang Harus Dijaga

Agar repo ini tetap sehat, dokumen yang benar-benar penting sebaiknya hanya:
- `README.md`
- `docs/standards/README.md`
- `docs/standards/authoring.md`
- `docs/repository-plan/README.md`
- `.cursorrules`
- `status.md`

Jika ada standar lain, fungsinya harus mendukung dokumen inti tersebut, bukan bersaing menjadi pusat aturan baru.
