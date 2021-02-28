use cmake;
use cmake::Config;

fn main() {
    if !cfg!(windows) {
        // stormlib has those builtin for windows
        // but we need to link them in on other platforms
        println!("cargo:rustc-link-lib=bz2");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=stdc++");
    }
    let dst = Config::new("StormLib")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=storm");
}