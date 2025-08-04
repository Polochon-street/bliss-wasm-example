use std::env;

fn main() {
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap().contains("wasm") {
        println!("cargo:rustc-link-arg=-sALLOW_MEMORY_GROWTH=1");
        println!("cargo:rustc-link-arg=-pthread");
        println!("cargo:rustc-link-arg=-sENVIRONMENT=web,worker,node");
        println!("cargo:rustc-link-arg=-sUSE_PTHREADS=1");
        println!("cargo:rustc-link-arg=-sWASM_WORKERS");
        println!("cargo:rustc-link-arg=-sPTHREAD_POOL_SIZE=12");
        println!("cargo:rustc-cfg=feature=\"wasm_threads\"");
        //println!(
        //    "cargo:rustc-env=RUSTFLAGS=-Ctarget-feature=+atomics,+bulk-memory,+mutable-globals"
        //);
    }
}
