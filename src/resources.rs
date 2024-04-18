use std::io::Write;
use std::{fs::File, path::Path};

use zip::ZipArchive;

use colored::Colorize;

pub fn check_resources() -> bool {
    return Path::new("./resources/dumpsxiso.exe").exists()
        && Path::new("./resources/mkpsxiso.exe").exists();
}

pub fn extract_archive() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("mkpsxiso.zip")?;
    let mut archive = ZipArchive::new(file)?;

    let target_files = &[
        "mkpsxiso-2.04-win64/bin/dumpsxiso.exe",
        "mkpsxiso-2.04-win64/bin/mkpsxiso.exe",
    ];
    for file_name in target_files {
        println!(
            "{}{} extracting file: {}...",
            "info".cyan().bold(),
            ":".bold(),
            file_name
        );

        let mut file = archive.by_name(file_name)?;
        let file_name = Path::new(file_name).file_name().unwrap().to_str().unwrap();
        let output_path = Path::new("resources").join(file_name);

        if let Some(p) = output_path.parent() {
            if !p.exists() {
                std::fs::create_dir_all(p)?;
            }
        }

        let mut outfile = File::create(output_path)?;
        std::io::copy(&mut file, &mut outfile)?;
    }

    Ok(())
}

pub fn download_resources() -> Result<bool, Box<dyn std::error::Error>> {
    let url =
        "https://github.com/Lameguy64/mkpsxiso/releases/download/v2.04/mkpsxiso-2.04-win64.zip";

    let response = reqwest::blocking::get(url)?;

    if !response.status().is_success() {
        eprintln!(
            "{}{} failed to download resources.",
            "error".red().bold(),
            ":".bold()
        );
        return Ok(false);
    }

    // download archive
    let mut file = File::create("mkpsxiso.zip")?;
    let body = response.bytes()?;
    let buffer: &[u8] = &body[..];
    file.write_all(buffer)?;

    // extract archive
    extract_archive()?;

    // delete archive
    std::fs::remove_file("mkpsxiso.zip").unwrap();

    Ok(true)
}
