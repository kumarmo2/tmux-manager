mod constructs;

use std::process::Command;

fn main() {
    // let window = configs::Window {};
    println!("Hello, world!");

    let output = Command::new("nohup")
        .args(vec!["tmux", "new-session", "-s", "some-session", "-d"])
        .output()
        .unwrap();

    if !output.status.success() {
        let err = String::from_utf8(output.stderr).unwrap();
        println!("err: {}", err);
        return;
    }
    println!(
        "success, out: {}",
        String::from_utf8(output.stdout).unwrap()
    )
}
