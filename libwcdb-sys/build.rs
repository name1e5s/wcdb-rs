use cmake::Config;

fn main() {
    let dst = Config::new("wrapper").build();
    let dst = dst.join("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=Security");

    println!("cargo:rustc-link-lib=static=WCDB");
    println!("cargo:rustc-link-lib=static=sqlcipher");
    println!("cargo:rustc-link-lib=static=zstd");
    // todo: multi platform
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=z");
}
