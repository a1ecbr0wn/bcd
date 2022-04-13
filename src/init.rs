use home::home_dir;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const BASH_INIT: &str = "eval \"$(bookmark-cd init)\"";

pub(crate) fn setup_shell() {
    touch_file();
    let (sh, _pid) = pshell::find().unwrap_or(("unknown".to_string(), 0));
    let shell_setup = match sh.as_str() {
        "bash" => check_bash(),
        _ => false,
    };
    if !shell_setup {
        println!("It looks like bookmark-cd (bcd) has not been set up to run in your shell [{}], do you want to set this up now? [Y/n]", sh);
        let mut reply = String::new();
        let _b = std::io::stdin().read_line(&mut reply).unwrap();
        reply = reply.trim().to_string();
        if reply.to_ascii_lowercase() == "y" || reply.to_ascii_lowercase() == "yes" {
            if sh.as_str() == "bash" {
                setup_bash();
            }
            println!("bookmark-cd (bcd) has now been set up in your shell as long as it is in your path, please restart your shell and use `bcd`");
        } else {
            println!("Setup cancelled");
        }
    }
}

fn check_bash() -> bool {
    let mut bashrc_file = home_dir().unwrap();
    bashrc_file.push(".bashrc");
    if bashrc_file.exists() {
        let file_res = File::open(bashrc_file);
        if let Ok(mut file) = file_res {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => contents.contains(BASH_INIT),
                Err(_) => {
                    println!("Cannot read `.bashrc` to install bookmark-cd (bcd) to run in your shell [bash]");
                    false
                }
            }
        } else {
            println!(
                "Cannot open `.bashrc` to install bookmark-cd (bcd) to run in your shell [bash]"
            );
            false
        }
    } else {
        println!("Cannot find `.bashrc` to install bookmark-cd (bcd) to run in your shell [bash]");
        false
    }
}

fn setup_bash() {
    let mut bashrc_file = home_dir().unwrap();
    bashrc_file.push(".bashrc");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(bashrc_file)
        .unwrap();

    writeln!(file).unwrap();
    writeln!(file, "# bookmark-cd init block").unwrap();
    writeln!(file, "{}", BASH_INIT).unwrap();
    writeln!(file).unwrap();
}

fn touch_file() {
    let mut bookmarks_file = home_dir().unwrap();
    bookmarks_file.push(".bcd");
    if !bookmarks_file.exists() {
        let _ = File::create(bookmarks_file);
    }
}

/// Outputs the shell script code for the function that will call this program, this should be used by an exec command
/// during shell initialisation, e.g. in .bashrc
pub(crate) fn initialise_shell_script() {
    println!("{}", include_str!("cmd.bash"));
}
