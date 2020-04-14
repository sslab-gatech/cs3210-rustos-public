pub fn main() {
    println!("cargo:rerun-if-changed=.cargo/layout.ld");
    println!("cargo:rerun-if-env-changed=VERBOSE_BUILD");
}
