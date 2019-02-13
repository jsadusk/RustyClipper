extern crate cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("libClipperHandle").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=ClipperHandle");
    if cfg!(target_os="macos"){
        println!("cargo:rustc-link-lib=dylib=c++");
    } else {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    println!("cargo:rustc-link-lib=dylib=polyclipping");
}
