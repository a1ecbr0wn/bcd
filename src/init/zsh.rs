use home::home_dir;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const ZSH_INIT: &str = "eval \"$(bookmark-cd init)\"";

pub(crate) fn instructions_zsh() {
    println!("To complete setup, please edit your ~/.zshrc file and insert the following to the end of the file:");
    println!("# bookmark-cd init block");
    println!("{}", ZSH_INIT);
}

pub(crate) fn check_zsh() -> bool {
    let mut zshrc_file = home_dir().unwrap();
    zshrc_file.push(".zshrc");
    if zshrc_file.exists() {
        let file_res = File::open(zshrc_file);
        if let Ok(mut file) = file_res {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => contents.contains(ZSH_INIT),
                Err(_) => {
                    println!("Cannot read `~/.zshrc` to install bookmark-cd (bcd) to run in your shell [zsh]");
                    false
                }
            }
        } else {
            println!(
                "Cannot open `~/.zshrc` to install bookmark-cd (bcd) to run in your shell [zsh]"
            );
            false
        }
    } else {
        println!("Cannot find `~/.zshrc` to install bookmark-cd (bcd) to run in your shell [zsh]");
        false
    }
}

pub(crate) fn setup_zsh(interactive: bool) {
    let mut zshrc_file = home_dir().unwrap();
    zshrc_file.push(".zshrc");
    if zshrc_file.exists() {
        let res = OpenOptions::new().write(true).append(true).open(zshrc_file);
        match res {
            Ok(mut file) => {
                writeln!(file).unwrap();
                writeln!(file, "# bookmark-cd init block").unwrap();
                writeln!(file, "{}", ZSH_INIT).unwrap();
                writeln!(file).unwrap();
                if interactive {
                    println!(
                        "Your shell init script has been set up, restart your shell and type `bcd`"
                    );
                }
            }
            Err(_) => {
                if interactive {
                    println!("Unable to open your shell init script.");
                }
            }
        }
    } else if interactive {
        println!(
            "Shell init script [{}] not found",
            zshrc_file.to_str().unwrap()
        );
    }
}
