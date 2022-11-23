fn main() {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(Eq)]")
        .out_dir("./src/pb")
        .compile_protos(&["./abi.proto"], &["./"])
        .unwrap();
    println!("cargo:rerun-if-changed=./abi.proto");
    println!("cargo:rerun-if-changed=./build.rs");
}
