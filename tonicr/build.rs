fn main() {
    tonic_build::configure()
        .type_attribute(".", "#[derive(Eq)]")
        .out_dir("src/tosei")
        .compile(&["./tosei.proto"], &["./"])
        .expect("compile proto file was failed");

    // もしそのファイルが変更したら、再ビルド
    println!("cargo:rerun-if-changed=./tosei.proto");
    println!("cargo:rerun-if-changed=./build.rs");
}
