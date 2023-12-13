use std::process::Command;
use std::time::Instant;
use glob::glob;

fn main() {
    for entry in glob("src/bin/*/").expect("failed to read src/bin") {
        match entry {
            Ok(path) => {
                if path.is_dir() {
                    let day = path.file_name().unwrap().to_str().unwrap();
                    let start = Instant::now();
                    let cmd = Command::new("cargo").args(["run", "--bin", day, "--release"]).output().unwrap();
                    let elapsed = start.elapsed();
                    let output = String::from_utf8(cmd.stdout).unwrap();
                    println!("Day {}:\n{}({:?} including build check)\n", day, output, elapsed);
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
