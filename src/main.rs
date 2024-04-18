use colored::Colorize;
use std::path::Path;
use std::process::Command;

mod resources;

use std::io::{self, BufRead};

fn extract_game(input_path: &Path) {
    let output_name = String::from("dump");

    let output = Command::new("./resources/dumpsxiso.exe")
        .arg(input_path)
        .arg("-x")
        .arg(&output_name)
        .arg("-s")
        .arg(output_name + "/structure.xml")
        .output()
        .expect("error: failed to execute dumpsxiso.");

    if !output.status.success() {
        eprintln!(
            "{}{} failed to extract game:\n\n{}",
            "error".red().bold(),
            ":".bold(),
            String::from_utf8_lossy(&output.stdout)
        );
        pause();
        return;
    }

    println!(
        "{}{} successfully extracted game.",
        "success".green().bold(),
        ":".bold()
    );
}

fn build_game(input_path: &Path) {
    let structure = input_path.join("structure.xml");
    let output_filename = String::from(input_path.file_name().unwrap().to_str().unwrap());

    let output = Command::new("./resources/mkpsxiso.exe")
        .arg(structure)
        .arg("-o")
        .arg(output_filename.clone() + ".bin")
        .arg("-c")
        .arg(output_filename + ".cue")
        .output()
        .expect("error: failed to execute mkpsxiso.");

    if !output.status.success() {
        eprintln!(
            "{}{} failed to build game:\n\n{}",
            "error".red().bold(),
            ":".bold(),
            String::from_utf8_lossy(&output.stdout)
        );
        pause();
        return;
    }

    println!(
        "{}{} successfully built game.",
        "success".green().bold(),
        ":".bold()
    );
}

fn pause() {
    let mut line = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_line(&mut line);
}

fn main() {
    if colored::control::set_virtual_terminal(true).is_err() {
        println!(
            "{}{} failed to set virtual terminal.",
            "warning".yellow().bold(),
            ":".bold()
        );
    }

    println!("PSXBUILD version 1.0-beta.1 - By SaturnKai");
    println!("MKPSXISO version 2.04 - Meido-Tek Productions (John \"Lameguy\" Wilbert Villamor/Lameguy64)\n\n");

    if !resources::check_resources() {
        println!(
            "{}{} downloading resources...",
            "info".cyan().bold(),
            ":".bold()
        );
        let result = resources::download_resources().expect("error: failed to download resources.");
        if !result {
            pause();
            return;
        }
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "{}{} you must pass in a dump folder or game file.",
            "error".red().bold(),
            ":".bold()
        );
        pause();
        return;
    }

    let input_path = Path::new(&args[1]);
    if !Path::exists(input_path) {
        eprintln!(
            "{}{} specified input path '{}' does not exist.",
            "error".red().bold(),
            ":".bold(),
            input_path.to_str().unwrap()
        );
        pause();
        return;
    }

    if Path::is_dir(input_path) {
        build_game(input_path);
    } else {
        extract_game(input_path);
    }
}
