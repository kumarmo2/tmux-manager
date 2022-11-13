mod constructs;

use constructs::session::Session;
use constructs::window::Window;
use constructs::State;

use std::fs;
const config_path: &str = "/home/manya/.config/tmux-manager/tmux-manager.yml";
// TODO: add check that windows name shouldn't be duplicate.
fn main() {
    let content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(err) => {
            println!("err while reading config, err: {}", err);
            return;
        }
    };

    println!("content: {}", content);
    let state = match serde_yaml::from_str::<State>(&content) {
        Err(err) => {
            println!("errr while deserializing config file: {}", err);
            return;
        }
        Ok(x) => x,
    };

    println!("state: {:?}", state);
    state.create().unwrap();

    // let mut session = Session::new("x".to_owned(), "/home/manya/configs".to_owned());

    // let window = Window::new("my-window".to_owned());
    // session.add_window(window);

    // let window = Window::new("my-window-2".to_owned());
    // session.add_window(window);

    // // let mut sessions = vec![session];

    // let mut state = State::with_session(session);
    // let mut session = Session::new("y".to_owned(), "/home/manya/configs".to_owned());

    // let window = Window::new("my-window".to_owned());
    // session.add_window(window);

    // let window = Window::new("my-window-2".to_owned());
    // session.add_window(window);

    // state.add_session(session);

    // let Err(err) =  state.create() else {
    // return;
    // };
    // println!(
    // "error occured while creating the tmux sessions, err: {}",
    // err
    // );
}
