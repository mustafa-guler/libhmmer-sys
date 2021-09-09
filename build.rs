use std::{path::Path, process::Command};

fn main() {
    // configure build
    //
    // the reason we're not using the `autotools` crate is we need a pretty particular setup:
    //
    // we only want to compile libhmmer.a and libeasel.a, none of the binaries but the makefile
    // that lets us do that is in `hmmer/src`, not `hmmer/`. This means that the `.make_target`
    // configuration doesn't work.
    //
    // Instead we'll just run subcommands
    // TODO: make sure autoconf and make exist and fail with informative error
    Command::new("autoconf")
        .current_dir("hmmer")
        .status()
        .expect("failed to autoconf");
    Command::new(std::fs::canonicalize("./hmmer/configure").unwrap())
        .current_dir("hmmer")
        .status()
        .expect("failed to configure");

    // compile libhmmer
    Command::new("make")
        .arg("-j")
        .arg("8")
        .arg("libhmmer.a")
        .current_dir("hmmer/src")
        .status()
        .expect("failed to build libhmmer.a");
    // compile libeasel
    Command::new("make")
        .arg("-j")
        .arg("8")
        .arg("libeasel.a")
        .current_dir("hmmer/easel")
        .status()
        .expect("failed to build libeasel.a");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    // generate rust bindings for only the functions we need
    // hmmer has a ton of `#include`s in the headers, meaning that we get a lot of libc in our
    // generated bindings unless we only `allowlist` exactly what we want
    // TODO: generate in out_dir so it doesn't pollute `src`
    bindgen::builder()
        .header("hmmer/src/hmmer.h")
        .clang_arg("-Ihmmer/easel")
        .clang_arg("-Ihmmer/src")
        // initialization of SIMD code
        .allowlist_function("impl_Init")
        // log sum table
        .allowlist_function("p7_FLogsumInit")
        // getopts
        .allowlist_function("esl_getopts_Create")
        .allowlist_function("esl_getopts_Destroy")
        // TODO: explain
        .allowlist_function("esl_sqfile_OpenDigital")
        .allowlist_function("esl_sq_CreateDigital")
        .allowlist_function("p7_bg_Create")
        .allowlist_function("p7_oprofile_ReadMSV")
        .allowlist_function("p7_hmmfile_OpenE")
        .allowlist_function("p7_hmmfile_Close")
        .allowlist_function("esl_sqio_Read")
        .allowlist_function("p7_tophits_Create")
        .allowlist_function("p7_tophits_Destroy")
        .allowlist_function("p7_pipeline_Create")
        .allowlist_function("p7_pipeline_Destroy")
        .allowlist_function("p7_pli_NewSeq")
        .allowlist_function("esl_sq_Reuse")
        .allowlist_type("P7_PIPELINE")
        .allowlist_type("P7_TOPHITS")
        .allowlist_function("p7_pli_NewModel")
        .allowlist_function("p7_bg_SetLength")
        .allowlist_function("p7_oprofile_ReconfigLength")
        .allowlist_function("p7_Pipeline")
        .allowlist_function("p7_oprofile_Destroy")
        .allowlist_function("p7_pipeline_Reuse")
        .allowlist_function("esl_alphabet_Destroy")
        .generate()
        .unwrap()
        .write_to_file("src/hmmer.rs")
        .unwrap();

    // copy static libs
    std::fs::copy("hmmer/src/libhmmer.a", out_dir.join("libhmmer.a")).unwrap();
    std::fs::copy("hmmer/easel/libeasel.a", out_dir.join("libeasel.a")).unwrap();

    // link both archives to our library
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=hmmer");
    println!("cargo:rustc-link-lib=static=easel");

    // uncomment once we're done developing
    // println!("cargo:rerun-if-changed=build.rs");
}
