use std::process::Command;

fn execute_command(command: &str, args: Vec<&str>) {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
}

fn create_namespace() {
    execute_command("ip", vec!["netns", "add", "my-namespace"]);
}

fn main() {

}
