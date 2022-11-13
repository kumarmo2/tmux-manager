use super::pane::Pane;

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
