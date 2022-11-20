use home::home_dir;
<<<<<<< HEAD
<<<<<<< HEAD
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
=======
=======
>>>>>>> b5a83dcfb041dd0876762bf4fa70a2f32c8abeba
use std::env;
use std::fs::File;
use std::process::exit;
>>>>>>> 8bb22af (Format Rust code using rustfmt)

<<<<<<< HEAD
mod snap;
use snap::check_in_snap;
=======
mod bash;
use bash::{check_bash, instructions_bash, setup_bash};

mod zsh;
use zsh::{check_zsh, instructions_zsh, setup_zsh};
>>>>>>> b5a83dcfb041dd0876762bf4fa70a2f32c8abeba

const SH_INIT: &str = "eval \"$(bookmark-cd init)\"";

// Check that the .bcd data file exists and the shell init script is setup
pub(crate) fn check_bookmarks_file() -> bool {
    let mut bookmarks_file = home_dir().unwrap();
    bookmarks_file.push(".bcd");
    bookmarks_file.exists()
}

<<<<<<< HEAD
=======
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
    let in_snap = env::var("SNAP_NAME").is_ok();
    (shell_name, shell_init_setup, in_snap)
}

>>>>>>> b5a83dcfb041dd0876762bf4fa70a2f32c8abeba
// Attempt to setup your shell, can be run in interactive mode or not, and exits the process if cancelled unexpectantly.
pub(crate) fn setup_shell(interactive: bool) -> bool {
    let bookmarks_file_exists = check_bookmarks_file();
    if !bookmarks_file_exists {
        let mut bookmarks_file = home_dir().unwrap();
        bookmarks_file.push(".bcd");
        if !bookmarks_file.exists() {
            let _ = File::create(bookmarks_file);
        }
    }

<<<<<<< HEAD
    let shell = ShellSetup::new();

    if !shell.shell_init_setup {
=======
    let (shell_name, shell_init_setup, in_snap) = check_shell();
    if !shell_init_setup {
>>>>>>> b5a83dcfb041dd0876762bf4fa70a2f32c8abeba
        if interactive {
            println!(
                "\nIt looks like bookmark-cd (bcd) has not been set up to run in your shell [{}].",
                shell.name
            );
<<<<<<< HEAD
            if shell.is_in_snap {
                println!(
                    "This may be because you have installed bcd from snap, which prevents automatic setup.\n"
                );
                if !shell.is_snap_connected {
                    println!("The snap container initially bocks access to shell init files that are needed to be checked for setup.  The following command can be run to unblock access to the required file and then try again:\n");
                    println!(
                        "    sudo snap connect bookmark-cd:{}\n \n",
                        shell.snap_connector
                    );
                }
                instructions_shell_script(shell.init);
                false
            } else {
                println!("Do you want to set this up now? [Y/n]");
                let mut reply = String::new();
                let _b = std::io::stdin().read_line(&mut reply).unwrap();
                reply = reply.trim().to_string();
                if reply.to_ascii_lowercase() == "y"
                    || reply.to_ascii_lowercase() == "yes"
                    || reply.is_empty()
                {
                    if shell.init.is_dir() {
                        println!(
                            "your shell [{}] is not currently supported",
                            shell.name.as_str()
                        );
                        false
                    } else {
                        setup_init_file(interactive, shell.init)
                    }
                } else {
                    println!("Setup cancelled");
                    exit(1);
                }
=======
            if in_snap {
                for (key, value) in env::vars() {
                    println!("{key}: {value}");
                }
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
>>>>>>> b5a83dcfb041dd0876762bf4fa70a2f32c8abeba
            }
        } else {
            setup_init_file(interactive, shell.init)
        }
    } else {
        true
    }
}

fn instructions_shell_script(init: PathBuf) {
    println!("To complete setup, please edit your [{}] file and insert the following to the end of the file:\n", init.to_str().unwrap());
    println!("    # bookmark-cd init block");
    println!("    {}", SH_INIT);
}

fn setup_init_file(interactive: bool, init_file: PathBuf) -> bool {
    if init_file.exists() {
        let res = OpenOptions::new().write(true).append(true).open(init_file);
        match res {
            Ok(mut file) => {
                writeln!(file).unwrap();
                writeln!(file, "# bookmark-cd init block").unwrap();
                writeln!(file, "{}", SH_INIT).unwrap();
                writeln!(file).unwrap();
                if interactive {
                    println!(
                        "Your shell init script has been set up, restart your shell and type `bcd`"
                    );
                }
                true
            }
            Err(_) => {
                println!("Please run bookmark-cd -i to setup");
                false
            }
        }
    } else if interactive {
        println!(
            "Shell init script [{}] not found",
            init_file.to_str().unwrap()
        );
        false
    } else {
        false
    }
}

#[allow(dead_code)]
/// Outputs the shell script code for the function that will call this program, this should be used by an exec command
/// during shell initialisation, e.g. in .bashrc
pub(crate) fn initialise_shell_script() {
    println!("{}", include_str!("_cmd.sh"));
}

struct ShellSetup {
    name: String,
    init: PathBuf,
    shell_init_setup: bool,
    is_in_snap: bool,
    is_snap_connected: bool,
    snap_connector: String,
}

impl ShellSetup {
    fn new() -> ShellSetup {
        let (is_in_snap, home_override) = check_in_snap();
        let (shell_name, _pid) = pshell::find().unwrap_or(("unknown".to_string(), 0));
        let mut shell_init = if let Some(home_override_path) = home_override {
            if home_override_path.is_dir() {
                home_override_path
            } else {
                home_dir().unwrap()
            }
        } else {
            home_dir().unwrap()
        };
        let mut snap_connector = String::new();
        match shell_name.as_str() {
            "bash" => {
                shell_init.push(".bashrc");
                if is_in_snap {
                    snap_connector = "dot-bashrc".to_string();
                }
            }
            "zsh" => {
                shell_init.push(".zshrc");
                if is_in_snap {
                    snap_connector = "dot-zshrc".to_string();
                }
            }
            _ => {}
        }
        let shell_init_exists = shell_init.exists();
        let shell_init_setup = if shell_init_exists {
            let file_res = File::open(shell_init.clone());
            if let Ok(mut file) = file_res {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => contents.contains(SH_INIT),
                    Err(_) => false,
                }
            } else {
                false
            }
        } else {
            false
        };
        let is_snap_connected = if is_in_snap {
            let snap_connected_status = Command::new("snapctl")
                .arg("is-connected")
                .arg(snap_connector.clone())
                .status()
                .unwrap_or_else(|_| {
                    panic!(
                        "Failed to check whether snap is able to read {}",
                        shell_init.clone().to_str().unwrap()
                    )
                });
            snap_connected_status.success()
        } else {
            false
        };
        ShellSetup {
            name: shell_name,
            init: shell_init,
            shell_init_setup,
            is_in_snap,
            is_snap_connected,
            snap_connector,
        }
    }
}
