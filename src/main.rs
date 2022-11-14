mod constructs;

use constructs::State;

// TODO: add check that windows name shouldn't be duplicate.

use std::fs;
const CONFIG_PATH: &str = "/home/manya/.config/tmux-manager/tmux-manager.yml";

fn main() {
    let content = match fs::read_to_string(CONFIG_PATH) {
        Ok(content) => content,
        Err(err) => {
            println!("err while reading config, err: {}", err);
            return;
        }
    };

    let state = match serde_yaml::from_str::<State>(&content) {
        Err(err) => {
            println!("errr while deserializing config file: {}", err);
            return;
        }
        Ok(x) => x,
    };

    state.create().unwrap();
}
