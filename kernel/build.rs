fn main() {
    println!("cargo:rustc-link-arg=-no-pie");
    println!("cargo:rustc-link-arg=--image-base=0xFFFFFFFFC0000000");

    #[cfg(feature = "test")]
    println!("cargo:rustc-cfg=testf");
}