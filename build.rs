use std::{process::Command, env::var};
use anyhow::Result;

use cmake;

fn go() -> Result<()> {
    let out_dir = var("OUT_DIR").unwrap();

    let xc_run_output = Command::new("xcrun").args(["-sdk", "macosx", "--show-sdk-path"]).output()?;
    let xc_sdk_path = std::str::from_utf8(&xc_run_output.stdout)?.trim();

    cmake::Config::new("aws-sdk-cpp")
    .always_configure(true)
    .define("CMAKE_COMPILE_WARNING_AS_ERROR", "OFF")
    .define("TARGET_ARCH", "APPLE")
    .define("CMAKE_BUILD_TYPE", "Debug")
    .define("BUILD_SHARED_LIBS", "ON")
    //.define("CMAKE_PREFIX_PATH", out_dir)//"$(ZLIB_OUTPUT);$(CURL_OUTPUT);$(OPENSSL_OUTPUT)")
    .define("BUILD_ONLY", "cognito-identity;cognito-idp")
    .define("ENABLE_TESTING", "OFF")
    .define("CUSTOM_MEMORY_MANAGEMENT", "OFF")
    .define("CMAKE_SHARED_LINKER_FLAGS", format!("-lz -F{}/System/Library/Frameworks -framework Security", xc_sdk_path))
    .define("CURL_INCLUDE_DIR", var("DEP_CURL_INCLUDE").unwrap())
    .define("CURL_LIBRARY", format!("{}/libcurl.a", var("DEP_CURL_LIB").unwrap()))
    .define("CMAKE_INSTALL_PREFIX", &out_dir)//env!("CARGO_MANIFEST_DIR"))
    .build();

    //println!("cargo:rustc-link-search=native={}", dst.display());
    //println!("cargo:rustc-link-lib=static=aws-sdk-cpp");
    println!("cargo:INCLUDE={}/include", out_dir);
    println!("cargo:LIB={}/lib", out_dir);
    println!("cargo:warning={}", out_dir);

    Ok(())
}

fn main() {
    if let Err(e) = go() {
        println!("cargo:warning={:?}", e);
    }
}