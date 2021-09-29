//! A light rust wrapper around [libhmmer](https://github.com/EddyRivasLab/hmmer) and [libeasel](https://github.com/EddyRivasLab/easel).

#![allow(
    clippy::all,
    unused,
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    deref_nullptr
)]

include!(concat!(env!("OUT_DIR"), "/hmmer.rs"));
