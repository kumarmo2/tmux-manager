use serde::{Deserialize, Serialize};
use std::process::Command;

use super::{pane::Pane, session::Session};

#[derive(Serialize, Deserialize, Debug)]
pub struct Window {
    pub name: Option<String>,
    pub root: Option<String>,
    pub panes: Option<Pane>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            name: None,
            root: None,
            panes: None,
        }
    }
}

impl Window {
    pub fn new(name: String) -> Self {
        Self {
            name: Some(name),
            root: None,
            panes: None,
        }
    }
    pub fn create(&self, session: &Session) {
        let mut args = vec!["tmux", "new-window"];
        if let Some(name) = self.name.as_ref() {
            args.push("-n");
            args.push(name);
        }
        args.push("-t");
        args.push(&session.name);

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
    }
}
