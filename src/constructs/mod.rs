use self::session::Session;

mod pane;
pub mod session;
pub mod window;

pub struct State {
    sessions: Vec<Session>,
}

impl State {
    pub fn new(sessions: Vec<Session>) -> Self {
        Self { sessions }
    }

    pub fn create(&self) {
        if self.sessions.is_empty() {
            return;
        }
        for session in &self.sessions {
            session.create();
        }
    }
}
