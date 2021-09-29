use std::{path::Path, process::Command};

fn main() {
    #[cfg(not(target_family = "unix"))]
    compile_error!("hmmer only supports unix so libhmmer-sys also only supports unix");
    
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

    let cpus = num_cpus::get().to_string();

    // compile libhmmer
    Command::new("make")
        .arg("-j")
        .arg(&cpus)
        .arg("libhmmer.a")
        .current_dir("hmmer/src")
        .status()
        .expect("failed to build libhmmer.a");
    // compile libeasel
    Command::new("make")
        .arg("-j")
        .arg(&cpus)
        .arg("libeasel.a")
        .current_dir("hmmer/easel")
        .status()
        .expect("failed to build libeasel.a");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    // generate rust bindings for only the functions we need
    //
    // hmmer has a ton of `#include`s in the headers, meaning that we get a lot of libc in our
    // generated bindings unless we only `allowlist` elements that are prefixed with "p7" for the
    // hmmer repo and "esl" for the easel repo
    bindgen::builder()
        .header("wrapper.h")
        .clang_arg("-Ihmmer/easel")
        .clang_arg("-Ihmmer/src")
        .allowlist_function("esl_.*")
        .allowlist_type("esl_.*")
        .allowlist_function("ESL_.*")
        .allowlist_type("ESL_.*")
        .allowlist_function("p7_.*")
        .allowlist_type("p7_.*")
        .allowlist_function("P7_.*")
        .allowlist_type("P7_.*")
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("hmmer.rs"))
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
