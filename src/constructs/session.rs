use std::process::Command;

use super::window::Window;

pub struct Session {
    name: Option<String>,
    windows: Option<Vec<Window>>,
    root: Option<String>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            name: None,
            windows: None,
            root: None,
        }
    }
}

impl Session {
    pub fn new(name: String, root: String) -> Self {
        // NOTE: for now only absoulute path seems to work.
        // Fix this if possible.
        Self {
            name: Some(name),
            windows: None,
            root: Some(root),
        }
    }
    pub fn create(&self) {
        let mut args = vec!["tmux", "new-session"];

        if let Some(directory) = self.root.as_ref() {
            args.push("-c");
            args.push(directory);
        }

        if let Some(name) = self.name.as_ref() {
            args.push("-s");
            args.push(name);
        }

        args.push("-d"); // NOTE: -d is important. if it is not given, it tries to attach to the
                         // ternimal, which is not present when we are using this binary to create the session.

        let output = Command::new("nohup").args(args).output().unwrap();
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
}
