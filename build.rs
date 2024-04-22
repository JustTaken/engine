fn main() {
    println!("cargo:rustc-link-search=/home/joao/.local/share/applications/engine/outputs/out/lib/");
    println!("cargo:rustc-link-lib=xdg");
    println!("cargo:rustc-link-lib=wayland-client");
}
