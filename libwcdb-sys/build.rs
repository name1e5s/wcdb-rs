use cmake::Config;

fn main() {
    let dst = Config::new("wrapper").build();
    let dst = dst.join("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=WCDB");
    // todo: multi platform
    println!("cargo:rustc-link-lib=c++");
}
