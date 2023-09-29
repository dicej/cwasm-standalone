fn main() {
    println!(r"cargo:rustc-link-search=/Users/ondrej/repos/cwasm-standalone/mylib/target/release/");
    println!(r"cargo:rustc-link-lib=mylib");
}
