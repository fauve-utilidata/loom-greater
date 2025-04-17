fn main() {
    println!("cargo:rustc-link-lib=dylib=loom");
    println!("cargo:rustc-link-search=native=../loom-cu/"); // Replace with the library's directory
}
