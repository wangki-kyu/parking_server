use std::io::Result;

fn main() -> Result<()> {
    // tonic_build를 사용하여 ocr.proto 파일을 컴파일합니다.
    tonic_prost_build::configure()
        .compile_protos(&["ocr.proto"], &["proto"])
        .unwrap();

    println!("cargo:rerun-if-changed=ocr.proto");
    Ok(())
}
