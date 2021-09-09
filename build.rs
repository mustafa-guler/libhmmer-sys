use std::{path::Path, process::Command};

fn main() {
    Command::new("autoconf")
        .current_dir("hmmer")
        .status()
        .expect("failed to autoconf");
    Command::new(std::fs::canonicalize("./hmmer/configure").unwrap())
        .current_dir("hmmer")
        .status()
        .expect("failed to configure");
    Command::new("make")
        .arg("-j")
        .arg("8")
        .arg("libhmmer.a")
        .current_dir("hmmer/src")
        .status()
        .expect("failed to build libhmmer.a");
    Command::new("make")
        .arg("-j")
        .arg("8")
        .arg("libeasel.a")
        .current_dir("hmmer/easel")
        .status()
        .expect("failed to build libeasel.a");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

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

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=hmmer");
    println!("cargo:rustc-link-lib=static=easel");

    // uncomment once we're done developing
    // println!("cargo:rerun-if-changed=build.rs");
}
