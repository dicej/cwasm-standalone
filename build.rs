fn main() {
    println!(r"cargo:rustc-link-search=/Users/ondrej/repos/cwasm-standalone/mylib/target/debug/");
    println!(r"cargo:rustc-link-lib=mylib");
}
