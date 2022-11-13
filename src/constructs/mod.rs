use self::session::Session;

mod pane;
pub mod session;
pub mod window;

pub struct State {
    sessions: Option<Vec<Session>>,
}

impl Default for State {
    fn default() -> Self {
        Self { sessions: None }
    }
}

impl State {
    pub fn with_session(session: Session) -> Self {
        let mut state = Self::default();
        state.add_session(session);
        state
    }
    pub fn add_session(&mut self, session: Session) {
        if let None = self.sessions {
            self.sessions = Some(Vec::new());
        }
        self.sessions.as_mut().unwrap().push(session);
    }

    pub fn create(&self) -> Result<(), String> {
        let Some(sessions) = &self.sessions else {
            return Err("Please provide some session".to_owned());
        };

        for session in sessions {
            session.create();
        }
        Ok(())
    }
}
