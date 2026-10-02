# ADR-0001: Rust sebagai core engine

## Status

Accepted

## Konteks

Verge membutuhkan engine penyimpanan dan query yang cepat, memory-safe, dapat
dikembangkan sendiri tanpa vendor lock-in, dan bisa dimasukkan ke kontainer tanpa
runtime bahasa yang besar. Spesifikasi produk menyebut "Rust core: performa
tinggi, aman, tanpa GC pause".

## Keputusan

Engine ditulis dalam Rust memakai toolchain bawaannya:

| Kebutuhan        | Tool                  |
| ----------------- | --------------------- |
| Build dan test    | `cargo`               |
| Format kode       | `rustfmt` (bawaan)    |
| Lint              | `clippy` (bawaan)     |
| Pin versi toolchain | `rust-toolchain.toml` |

Versi Rust dipin ke `1.82`, MSRV crate disamakan dengan versi itu, dan seluruh
crate diwajibkan memakai `forbid(unsafe_code)`.

## Alternatif yang dipertimbangkan

- **Zig** — toolchain sangat baik karena build, test, dan format semuanya bawaan.
  Ditolak karena ekosistem pustakanya masih jauh lebih tipis untuk kebutuhan
  storage dan serialisasi, serta banyak memuat `unsafe` di pustaka standarnya.
- **Go** — matang dan mudah dipelajari. Ditolak karena orientasi garbage
  collector dan pertumbuhan heap pada beban query panjang.
- **C++** — performa setara, tetapi risiko keamanan jauh lebih tinggi dan
  kebutuhan tooling pendukung jauh lebih besar.

## Konsekuensi

- Build cepat, dependensi native minim, binary kecil.
- Tidak ada script custom untuk quality gate: semuanya memakai tool resmi.
- Kurva belajar Rust; diterima sebagai risiko yang perlu disadari.

## Justifikasi

Rust adalah bahasa produksi mainstream yang memberi memory safety tanpa garbage
collector, dengan quality tooling bawaan dan ekosistem crate yang besar.

## Tanggal

2026-10-03

## Penulis

Miruameli
