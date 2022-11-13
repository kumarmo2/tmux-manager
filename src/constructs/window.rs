use super::pane::Pane;

pub struct Window {
    name: Option<String>,
    root: Option<String>,
    panes: Option<Pane>,
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
