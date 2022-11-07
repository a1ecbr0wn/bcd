use home::home_dir;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const BASH_INIT: &str = "eval \"$(bookmark-cd init)\"";

pub(crate) fn check_bash() -> bool {
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

pub(crate) fn setup_bash(interactive: bool) {
    let mut bashrc_file = home_dir().unwrap();
    bashrc_file.push(".bashrc");
    if bashrc_file.exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(bashrc_file)
            .unwrap();

        writeln!(file).unwrap();
        writeln!(file, "# bookmark-cd init block").unwrap();
        writeln!(file, "{}", BASH_INIT).unwrap();
        writeln!(file).unwrap();
        if interactive {
            println!("Your shell init script has been set up, restart your shell and type `bcd`");
        }
    } else {
        if interactive {
            println!(
                "Shell init script [{}] not found",
                bashrc_file.to_str().unwrap()
            );
        }
    }
}
