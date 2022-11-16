use home::home_dir;
use std::fs::File;
use std::process::exit;
use std::env;

mod bash;
use bash::{check_bash, instructions_bash, setup_bash};

mod zsh;
use zsh::{check_zsh, instructions_zsh, setup_zsh};

const SH_INIT: &str = "eval \"$(bookmark-cd init)\"";

// Check that the .bcd data file exists and the shell init script is setup
pub(crate) fn check_bookmarks_file() -> bool {
    let mut bookmarks_file = home_dir().unwrap();
    bookmarks_file.push(".bcd");
    bookmarks_file.exists()
}

// Check that the shell init script is setup
pub(crate) fn check_shell() -> (String, bool, bool) {
    let (shell_name, _pid) = pshell::find().unwrap_or(("unknown".to_string(), 0));
    let shell_init_setup = match shell_name.as_str() {
        "bash" => check_bash(),
        "zsh" => check_zsh(),
        _ => {
            println!("your shell [{}] is not currently supported, the following needs to be set in your shell init script:", shell_name.as_str());
            println!("{}", SH_INIT);
            true
        }
    };
    (shell_name, shell_init_setup, true)
}

// Attempt to setup your shell, can be run in interactive mode or not, and exits the process if cancelled unexpectantly.
pub(crate) fn setup_shell(interactive: bool) {
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
    let bookmarks_file_exists = check_bookmarks_file();
    if !bookmarks_file_exists {
        let mut bookmarks_file = home_dir().unwrap();
        bookmarks_file.push(".bcd");
        if !bookmarks_file.exists() {
            let _ = File::create(bookmarks_file);
        }
    }

    let (shell_name, shell_init_setup, in_snap) = check_shell();
    if !shell_init_setup {
        if interactive {
            println!(
                "It looks like bookmark-cd (bcd) has not been set up to run in your shell [{}].",
                shell_name
            );
            if in_snap {
                println!(
                    "You appear to have installed bcd from snap, so this cannot be set up automatically."
                );
                match shell_name.as_str() {
                    "bash" => instructions_bash(),
                    "zsh" => instructions_zsh(),
                    _ => {}
                }
            } else {
                println!("Do you want to set this up now? [Y/n]");
                let mut reply = String::new();
                let _b = std::io::stdin().read_line(&mut reply).unwrap();
                reply = reply.trim().to_string();
                if reply.to_ascii_lowercase() == "y"
                    || reply.to_ascii_lowercase() == "yes"
                    || reply.is_empty()
                {
                    match shell_name.as_str() {
                        "bash" => {
                            setup_bash(interactive);
                        }
                        "zsh" => {
                            setup_zsh(interactive);
                        }
                        _ => {
                            println!(
                                "your shell [{}] is not currently supported",
                                shell_name.as_str()
                            );
                        }
                    }
                } else {
                    println!("Setup cancelled");
                    exit(1);
                }
            }
        } else {
            match shell_name.as_str() {
                "bash" => setup_bash(interactive),
                "zsh" => setup_zsh(interactive),
                _ => {}
            }
        }
    }
}

#[allow(dead_code)]
/// Outputs the shell script code for the function that will call this program, this should be used by an exec command
/// during shell initialisation, e.g. in .bashrc
pub(crate) fn initialise_shell_script() {
    println!("{}", include_str!("_cmd.sh"));
}
