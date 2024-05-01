
use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("Command executed successfully:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("Command failed:\n{}", s);
    }
}