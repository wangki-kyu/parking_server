use std::{fs::{self, File}, path::Path};

use anyhow::bail;
use base64::{engine::general_purpose, Engine};

fn main() {
     // ⚠️ 주의: 이 파일 경로를 실제 사용자의 환경에 맞게 변경해야 합니다.
     let image_file_path = "./assets/plate.png"; 
    
     println!("Reading file: {}", image_file_path);
 
     match read_image_to_raw_bytes(image_file_path) {
         Ok(raw_image_bytes) => {
             println!("\n✅ Successfully read raw image data.");
             println!("   Total size (bytes): {}", raw_image_bytes.len());
             
             // raw_image_bytes 변수에 이미지의 순수한 바이트 데이터가 담겨 있습니다.
             // 이 데이터를 Postman으로 전송하려면 JSON에 담기 위해 Base64 인코딩이 필요합니다.
             // (Base64 인코딩을 안 하고 전송하는 것은 HTTP/JSON 표준으로는 불가능합니다.)
 
             // 만약 디버깅 목적으로 바이트의 일부를 확인하고 싶다면:
            //  let bytes_to_show = 20;
            //  println!("   First {} bytes (Hex): {:?}", bytes_to_show, &raw_image_bytes[0..bytes_to_show]);
            let base64_string = general_purpose::STANDARD.encode(&raw_image_bytes);
            println!("base64: {}", base64_string);
         }
         Err(e) => {
             eprintln!("\n❌ Failed to read image file: {}", e);
         }
     }
}

fn read_image_to_raw_bytes(file_path: &str) -> Result<Vec<u8>, String> {
    // 1. 파일 경로가 유효한지 확인합니다.
    if !Path::new(file_path).exists() {
        return Err(format!("Error: File not found at path: {}", file_path));
    }

    // 2. fs::read 함수를 사용하여 파일을 한 번에 읽어 바이트 배열(Vec<u8>)로 가져옵니다.
    // 이 바이트 배열이 이미지의 원시(Raw) 데이터입니다.
    match fs::read(file_path) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(format!("Error reading file: {}", e)),
    }
}