use std::{env, io};

use crate::fs_tools::dirs;

pub fn cmd_init() {
    match dirs::create_repo_dir() {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == io::ErrorKind::AlreadyExists {
                eprintln!("this repo has initialized.");
                return;
            } else {
                eprintln!("init error: {}", e);
                return;
            }
        }
    }

    match dirs::create_objects_dir() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("init objects dir error: {}", e);
            return;
        }
    }

    let cwd = env::current_dir().unwrap();

    println!(
        "Initialized empty ugit repository in {}/{}",
        cwd.to_str().unwrap(),
        dirs::UGIT_REPOSITORY_NAME
    );
}
