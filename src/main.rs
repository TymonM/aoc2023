use glob::glob;
use std::process::Command;
use std::time::Instant;

fn main() {
    for entry in glob("src/bin/*/").expect("failed to read src/bin") {
        match entry {
            Ok(path) => {
                if path.is_dir() {
                    let day = path.file_name().unwrap().to_str().unwrap();
                    let _build = Command::new("cargo")
                        .args(["build", "--bin", day, "--release"])
                        .output()
                        .unwrap();
                    let start = Instant::now();
                    let runcmd = Command::new("sh")
                        .args(["-c", &format!("./target/release/{}", day)])
                        .output()
                        .unwrap();
                    let elapsed = start.elapsed();
                    let output = String::from_utf8(runcmd.stdout).unwrap();
                    println!("Day {}:\n{}{:?}\n", day, output, elapsed);
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
