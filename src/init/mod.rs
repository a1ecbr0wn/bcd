use home::home_dir;
use std::fs::File;

mod bash;
use bash::{check_bash, setup_bash};

mod zsh;
use zsh::{check_zsh, setup_zsh};

const SH_INIT: &str = "eval \"$(bookmark-cd init)\"";

pub(crate) fn check_shell() -> (String, bool) {
    let (shell_name, _pid) = pshell::find().unwrap_or(("unknown".to_string(), 0));
    let setup = match shell_name.as_str() {
        "bash" => check_bash(),
        "zsh" => check_zsh(),
        _ => {
            println!("your shell [{}] is not currently supported, the following needs to be set in your shell init script:", shell_name.as_str());
            println!("{}", SH_INIT);
            true
        }
    };
    (shell_name, setup)
}

pub(crate) fn setup_shell(interactive: bool) {
    touch_file();
    let (shell_name, shell_setup) = check_shell();
    if !shell_setup {
        if interactive {
            println!("It looks like bookmark-cd (bcd) has not been set up to run in your shell [{}], do you want to set this up now? [Y/n]", shell_name);
            let mut reply = String::new();
            let _b = std::io::stdin().read_line(&mut reply).unwrap();
            reply = reply.trim().to_string();
            if reply.to_ascii_lowercase() == "y" || reply.to_ascii_lowercase() == "yes" {
                if shell_name.as_str() == "bash" {
                    setup_bash();
                    println!("bookmark-cd (bcd) has now been set up in your shell as long as it is in your path, please restart your shell and use `bcd`");
                } else if shell_name.as_str() == "zsh" {
                    setup_zsh();
                    println!("bookmark-cd (bcd) has now been set up in your shell as long as it is in your path, please restart your shell and use `bcd`");
                } else {
                    println!(
                        "your shell [{}] is not currently supported",
                        shell_name.as_str()
                    );
                }
            } else {
                println!("Setup cancelled");
            }
        } else {
            match shell_name.as_str() {
                "bash" => setup_bash(),
                "zsh" => setup_zsh(),
                _ => {}
            }
        }
    }
}

fn touch_file() {
    let mut bookmarks_file = home_dir().unwrap();
    bookmarks_file.push(".bcd");
    if !bookmarks_file.exists() {
        let _ = File::create(bookmarks_file);
    }
}

#[allow(dead_code)]
/// Outputs the shell script code for the function that will call this program, this should be used by an exec command
/// during shell initialisation, e.g. in .bashrc
pub(crate) fn initialise_shell_script() {
    println!("{}", include_str!("_cmd.sh"));
}
