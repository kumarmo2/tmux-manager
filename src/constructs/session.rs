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
