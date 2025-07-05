use std::process::Command;
use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let asm_dir = Path::new("asm");
    let out_dir_path = env::var("PWD").expect("failed to get PWD env") + "/output";
    let out_dir = Path::new(out_dir_path.as_str());

    // Make sure output dir exists
    fs::create_dir_all(&out_dir).unwrap();

    // Collect .S files and assemble them
    for entry in fs::read_dir(asm_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("S") {
            let filename = path.file_stem().unwrap().to_str().unwrap();
            let out_file = out_dir.join(format!("{}.o", filename));

            let status = Command::new("aarch64-none-elf-as")
                .args(&["-g", "-o"])
                .arg(&out_file)
                .arg(&path)
                .status()
                .expect("Failed to run assembler");
            assert!(status.success());

            println!("cargo:rerun-if-changed={}", path.display());
            println!("cargo:rustc-link-arg={}", out_file.display());
        }
    }
}