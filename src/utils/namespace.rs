use std::process::Command;

pub fn create_namespace(filesystem: &str) -> u32 {
    let child = Command::new("unshare")
        .arg("-p")
        .arg("-f")
        .arg(&format!("--mount-proc={}/proc", filesystem))
        .spawn()
        .expect("Failed to create namespace");

    return child.id()
}