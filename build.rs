fn main() {
    let libc_path = std::env::var("AVR_LIBC_PATH").unwrap_or_default();
    if !libc_path.is_empty() {
        // Tell GCC where to find the library directory 
        println!("cargo:rustc-link-arg=-B{}/avrxmega3/short-calls", libc_path);
        println!("cargo:rustc-link-search=native={}/avrxmega3/short-calls", libc_path);
    }

    println!("cargo:rerun-if-env-changed=AVR_LIBC_PATH");
    println!("cargo:rerun-if-changed=build.rs");
}