fn main() {
    println!("cargo:rustc-link-arg=/SUBSYSTEM:windows,4.0");
    println!("cargo:rustc-link-arg=/ENTRY:WinMain");

    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=vc6-sys-extra");
}
