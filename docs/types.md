---
title: Types | ImageTools Docs
---

[Home](https://19wintersp.github.io/ImageTools/)

# Types

Some options require specific values.

## Values

### String

Strings are UTF-8 strings.

More information at [doc.rust-lang.org](https://doc.rust-lang.org/std/string/struct.String.html).

### Integer

Integers are usually 32-bit or 64-bit signed or unsigned integers, defined as Rust `i32` and `i64` (signed) and `u32` and `u64` (unsigned). They are parsed with their `FromStr` implementations.

### Float

Floats are either 32-bit or 64-bit floating points, defined as Rust `f32` and `f64` respectively, and parsed with their `FromStr` implementations. Valid strings include:

* `3.14`
* `-3.14`
* `2.5e10`
* `2.5E10`
* `2.5e-10`
* `2.5E-10`
* `5.`
* `.5`
* `0.5`
* `inf`
* `-inf`
* `NaN`

Note however that not all of these will be valid in specific contexts.

More information at [doc.rust-lang.org](https://doc.rust-lang.org/std/primitive.f64.html#method.from_str).

### Color

Colors are 8-bit 4-channel colors stored as RGBA. They are parsed by [the `color_processing` crate](https://crates.io/crates/color_processing). The strings are parsed as CSS colors. Valid strings include:

* `red`
* `#f00`
* `#F00`
* `#f00f`
* `#ff0000`
* `#ff0000ff`
* `rgb(255, 0, 0)`
* `rgba(255, 0, 0, 255)`
* `hsl(0, 100%, 50%)`
* `hsv(0, 100%, 100%)`

More information at [docs.rs](https://docs.rs/color_processing/0.6.0/color_processing/).

## Enums

### Image format

Names are case-insensitive.

#### `png`

PNG image

#### `gif`

GIF image

#### `webp`

WebP image

#### `tiff`

TIFF image

#### `tga`

TGA image

#### `dds`

DDS image

#### `hdr`

HDR image

#### `avif`

AVIF image

#### `jpeg`, `jpg`

JPEG image

#### `bmp`, `bitmap`

Device Independent Bitmap image

#### `ico`, `icon`

Windows icon file

#### `pnm`, `pgm`, `pbm`, `pam`

PNM-class image

#### `ff`, `farbfeld`

Farbfeld image

### Image output format

Names are case-insensitive.

#### `png`

PNG image

#### `jpeg`, `jpg`

JPEG image

#### `gif`

GIF image

#### `ico`

Windows icon file

#### `bmp`

Device Independent Bitmap image

#### `ff`, `farbfeld`

Farbfeld image

#### `tga`

TGA image

#### `pnmb`, `pbm`, `pnmb-b`, `pbm-b`, `p1`, `pnm1`

Binary PNM Bitmap image

#### `pnmb-a`, `pbm-a`, `p4`, `pnm4`

ASCII PNM Bitmap image

#### `pnmg`, `pgm`, `pnmg-b`, `pgm-b`, `p2`, `pnm2`

Binary PNM Graymap image

#### `pnmg-a`, `pgm-a`, `p5`, `pnm5`

ASCII PNM Graymap image

#### `pnmp`, `ppm`, `pnmp-b`, `ppm-b`, `p3`, `pnm3`

Binary PNM Pixmap image

#### `pnmp-a`, `ppm-a`, `p6`, `pnm6`

ASCII PNM Pixmap image

#### `pnma`, `pam`, `p7`, `pnm7`

PNM Arbitrary Map image

### Filter

Names are case-insensitive.

#### `nearest`

Nearest Neighbor filter

#### `triangle`

Linear Triangle filter

#### `gaussian`

Gaussian filter

#### `catmull`, `catmullrom`

Cubic Catmull-Rom filter

#### `lanczos3`

Lanczos (window 3) filter

### Color format

Names are case-insensitive.

#### `hex`

Hex color expression

#### `cmyk`

CMYK color function

#### `hsl`, `hsla`

HSL color function

#### `hsv`, `hsva`

HSV color function

#### `hwb`, `hwba`

HWB color function

#### `rgb`, `rgba`

RGB color function
