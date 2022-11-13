mod constructs;

use constructs::session::Session;
use constructs::window::Window;
use constructs::State;

// TODO: add check that windows name shouldn't be duplicate.
fn main() {
    let mut session = Session::new("x".to_owned(), "/home/manya/configs".to_owned());

    let window = Window::new("my-window".to_owned());
    session.add_window(window);

    let window = Window::new("my-window-2".to_owned());
    session.add_window(window);

    let mut sessions = vec![session];

    let mut session = Session::new("y".to_owned(), "/home/manya/configs".to_owned());

    let window = Window::new("my-window".to_owned());
    session.add_window(window);

    let window = Window::new("my-window-2".to_owned());
    session.add_window(window);

    sessions.push(session);
    let state = State::new(sessions);

    state.create();
}
