use std::{process::Command, env::var, path::Path};
use anyhow::Result;
use cmake;

fn go() -> Result<()> {
    let out_dir = var("OUT_DIR").unwrap();
    match var("TARGET") {
        Ok(target) if target == "aarch64-linux-android" => {
            match var("HOST") {
                Ok(host) if host == "x86_64-apple-darwin" => {
                    assert!(Path::new(&format!("{}/libcurl.a", var("DEP_CURL_LIB").unwrap())).exists());
                    assert!(Path::new(&format!("{}/libcrypto.a", var("DEP_OPENSSL_LIB").unwrap())).exists());
                    assert!(Path::new(&format!("{}/libssl.a", var("DEP_OPENSSL_LIB").unwrap())).exists());
                    let ndk_root = var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME is undefined");

                    cmake::Config::new("aws-sdk-cpp")
                        .always_configure(true)
                        .define("TARGET_ARCH", "ANDROID")
                        .define("NDK_DIR", ndk_root.as_str())
                        .define("ANDROID_ABI", "arm64-v8a")
                        .define("CMAKE_BUILD_TYPE", if var("PROFILE").unwrap() == "debug" { "Debug" } else { "Release" })
                        .define("BUILD_SHARED_LIBS", "ON")
                        .define("BUILD_ONLY", "cognito-identity;cognito-idp")
                        .define("ENABLE_TESTING", "OFF")
                        .define("CUSTOM_MEMORY_MANAGEMENT", "ON")
                        .define("ANDROID_NATIVE_API_LEVEL", "24")
                        //.define("ANDROID_STL", "c++_shared")
                        .define("ANDROID_STL", "gnustl_shared")
                        .define("ANDROID_BUILD_CURL", "OFF")
                        .define("ANDROID_BUILD_OPENSSL", "OFF")
                        .define("ANDROID_BUILD_ZLIB", "OFF")
                        .define("CURL_INCLUDE_DIR", var("DEP_CURL_INCLUDE").unwrap())
                        .define("CURL_LIBRARY", format!("{}/libcurl.a", var("DEP_CURL_LIB").unwrap()))
                        .define("OPENSSL_INCLUDE_DIR", var("DEP_OPENSSL_INCLUDE").unwrap())
                        .define("OPENSSL_CRYPTO_LIBRARY", format!("{}/libcrypto.a", var("DEP_OPENSSL_LIB").unwrap()))
                        .define("OPENSSL_SSL_LIBRARY", format!("{}/libssl.a", var("DEP_OPENSSL_LIB").unwrap()))
                        .define("CMAKE_INSTALL_PREFIX", &out_dir) //env!("CARGO_MANIFEST_DIR"))
                        .build();
                }
                x => panic!("{:?} not yet supported", x),
            }
        }
        _ => {
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
                .define("CMAKE_INSTALL_PREFIX", &out_dir) //env!("CARGO_MANIFEST_DIR"))
                .build();

            //println!("cargo:rustc-link-search=native={}", dst.display());
            //println!("cargo:rustc-link-lib=static=aws-sdk-cpp");
        }
    }

    println!("cargo:INCLUDE={}/include", out_dir);
    println!("cargo:LIB={}/lib", out_dir);

    Ok(())
}

fn main() {
    if let Err(e) = go() {
        println!("cargo:warning={:?}", e);
    }
}
