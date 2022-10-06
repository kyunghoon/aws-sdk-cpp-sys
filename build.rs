use std::process::Command;
use anyhow::Result;

use cmake;

fn go() -> Result<()> {
    /*-DTARGET_ARCH=APPLE \
    -DCMAKE_BUILD_TYPE=Debug \
    -DBUILD_SHARED_LIBS=ON \
    -DCMAKE_PREFIX_PATH="$(ZLIB_OUTPUT);$(CURL_OUTPUT);$(OPENSSL_OUTPUT)" \
    -DBUILD_ONLY="cognito-identity;cognito-idp" \
    -DENABLE_TESTING=OFF \
    -DCUSTOM_MEMORY_MANAGEMENT=OFF \
    -DCMAKE_SHARED_LINKER_FLAGS="-lz -F$(shell xcrun -sdk macosx --show-sdk-path)/System/Library/Frameworks -framework Security" \
    -DCURL_INCLUDE_DIR=$(CURL_OUTPUT)/include \
    -DCURL_LIBRARY=$(CURL_OUTPUT)/lib/libcurl.a \
    -DCMAKE_COMPILE_WARNING_AS_ERROR=OFF*/

    let output = Command::new("xcrun").args(["-sdk", "macosx", "--show-sdk-path"]).output()?;
    let xc_sdk_path = std::str::from_utf8(&output.stdout)?.trim();

    let curl_output_dir = ".".to_string();// var_os("DEP_CURL_OUTPUT").and_then(|s| s.to_str().map(|s| s.to_string())).unwrap();

    let dst = cmake::Config::new("aws-sdk-cpp")
    .always_configure(true)
    .define("CMAKE_COMPILE_WARNING_AS_ERROR", "OFF")
    .define("TARGET_ARCH", "APPLE")
    .define("CMAKE_BUILD_TYPE", "Debug")
    .define("BUILD_SHARED_LIBS", "ON")
    //.define("CMAKE_PREFIX_PATH", "$(ZLIB_OUTPUT);$(CURL_OUTPUT);$(OPENSSL_OUTPUT)")
    .define("BUILD_ONLY", "cognito-identity;cognito-idp")
    .define("ENABLE_TESTING", "OFF")
    .define("CUSTOM_MEMORY_MANAGEMENT", "OFF")
    .define("CMAKE_SHARED_LINKER_FLAGS", format!("-lz -F{}/System/Library/Frameworks -framework Security", xc_sdk_path))
    .define("CURL_INCLUDE_DIR", format!("{}/include", curl_output_dir))
    .define("CURL_LIBRARY", format!("{}/lib/libcurl.a", curl_output_dir))
    .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=aws-sdk-cpp");

    Ok(())
}

fn main() {
    if let Err(e) = go() {
        println!("cargo:warning={:?}", e);
    }
}