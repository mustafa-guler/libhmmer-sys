# libhmmer-sys
Rust bindings to libhmmer and libeasel. Please refer to the [original C codebase](https://github.com/EddyRivasLab/hmmer) for documentation.

## Building
Requires `autoconf`, `make`, and a `C` compiler to build.
```
git clone --recursive https://github.com/mustafa-guler/libhmmer-sys
cargo build
```

## Licensing
Since this is an extremely light wrapper around [`hmmer`](https://github.com/EddyRivasLab/hmmer) this preserves the [same license](https://github.com/EddyRivasLab/hmmer/blob/master/LICENSE).
