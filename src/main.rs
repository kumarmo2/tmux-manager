mod constructs;

use constructs::session::Session;
use std::process::Command;

fn main() {
    let session = Session::new("x".to_owned(), "/home/manya/configs".to_owned());
    // let session = Session::new("x".to_owned(), "~/configs".to_owned());
    session.create();
}
