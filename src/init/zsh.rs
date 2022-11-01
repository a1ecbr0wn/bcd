use home::home_dir;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const ZSH_INIT: &str = "eval \"$(bookmark-cd init)\"";

pub(crate) fn check_zsh() -> bool {
    let mut zshrc_file = home_dir().unwrap();
    zshrc_file.push(".zshrc");
    if zshrc_file.exists() {
        let file_res = File::open(zshrc_file);
        if let Ok(mut file) = file_res {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    let rtn = contents.contains(ZSH_INIT);
                    if rtn == true {
                        println!("zsh set up for bcd");
                    }
                    rtn
                }
                Err(_) => {
                    println!("Cannot read `.zshrc` to install bookmark-cd (bcd) to run in your shell [zsh]");
                    false
                }
            }
        } else {
            println!(
                "Cannot open `.zshrc` to install bookmark-cd (bcd) to run in your shell [zsh]"
            );
            false
        }
    } else {
        println!("Cannot find `.zshrc` to install bookmark-cd (bcd) to run in your shell [zsh]");
        false
    }
}

pub(crate) fn setup_zsh() {
    let mut zshrc_file = home_dir().unwrap();
    zshrc_file.push(".zshrc");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(zshrc_file)
        .unwrap();

    writeln!(file).unwrap();
    writeln!(file, "# bookmark-cd init block").unwrap();
    writeln!(file, "{}", ZSH_INIT).unwrap();
    writeln!(file).unwrap();
}