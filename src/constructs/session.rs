use std::process::Command;

use super::window::Window;

pub struct Session {
    pub name: String,
    windows: Option<Vec<Window>>,
    root: Option<String>,
    default_window_name: String,
}

impl Session {
    pub fn new(name: String, root: String) -> Self {
        // NOTE: for now only absoulute path seems to work.
        // Fix this if possible.
        Self {
            name,
            windows: None,
            root: Some(root),
            default_window_name: "first".to_owned(),
        }
    }
    pub fn add_window(&mut self, window: Window) {
        if let None = self.windows {
            self.windows = Some(vec![]);
        }
        self.windows.as_mut().unwrap().push(window);
    }
    pub fn create(&self) {
        /*
         * Commands
         *   - tmux new-session -s x -d
         *   - tmux new-session -s x -n sdfdsf -d
         *
         *   - tmux new-session -s x -n sdfdsf -d
         * */
        let mut args = vec!["tmux", "new-session"];

        if let Some(directory) = self.root.as_ref() {
            args.push("-c");
            args.push(directory);
        }

        args.push("-s");
        args.push(self.name.as_ref());
        args.push("-n");
        args.push(&self.default_window_name);

        // NOTE: -d is important. if it is not given, it tries to attach to the
        // ternimal, which is not present when we are using this binary to create the session.
        args.push("-d");

        let output = Command::new("nohup").args(args).output().unwrap();
        if !output.status.success() {
            let err = String::from_utf8(output.stderr).unwrap();
            println!("err: {}", err);
            return;
        }
        println!(
            "success, out: {}",
            String::from_utf8(output.stdout).unwrap()
        );

        let Some(windows) = self.windows.as_ref() else {
            return;
        };
        if windows.len() == 0 {
            return;
        }

        for window in windows {
            window.create(self);
        }

        // By default, when a new-session is created, a window is created.
        // Now if windows are present in the Session.windows, we are creating that
        // many windows. So in that case, we are deleting the default window created.
        // we have provided the default name for that window while creating the session
        // so as it is easy to target it in the commands.

        let mut window_target = self.name.to_owned();
        window_target.push_str(":");
        window_target.push_str(&self.default_window_name);

        let args = vec!["tmux", "kill-window", "-t", &window_target];
        Command::new("nohup").args(args).output().unwrap();
    }
}
