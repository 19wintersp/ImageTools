---
title: Image Formats
---

[Home](https://19wintersp.github.io/ImageTools/)

# Image Formats

Only certain image formats are supported by the [`image` crate](https://crates.io/crates/image) backend.

| Format   | Decoding                  | Encoding                   |
| -------- | ------------------------- | -------------------------- |
| PNG      | All supported color types | Same as decoding           |
| JPEG     | Baseline and progressive  | Baseline JPEG              |
| GIF      | Yes                       | Yes                        |
| BMP      | Yes                       | RGB(A)8 Gray(A)8           |
| ICO      | Yes                       | Yes                        |
| TIFF     | Baseline + LZW + PackBits | RGB(A)8, Gray8             |
| WebP     | Lossy (Luma channel only) | No                         |
| AVIF     | Only 8-bit                | Lossy                      |
| PNM      | PBM, PGM, PPM, PAM        | Yes                        |
| DDS      | DXT1, DXT3, DXT5          | No                         |
| TGA      | Yes                       | RGB(A)8, BGR(A)8, Gray(A)8 |
| Farbfeld | Yes                       | Yes                        |

More information at [docs.rs](https://docs.rs/image/0.23.14/image/codecs/).
