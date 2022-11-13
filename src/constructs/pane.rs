use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pane {
    name: Option<String>,
}

impl Default for Pane {
    fn default() -> Self {
        Self { name: None }
    }
}
