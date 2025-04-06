use home::home_dir;
use snapcraft::{check_snap_home, snap_data};
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{prelude::*, stdout, ErrorKind},
    path::PathBuf,
    process::{exit, Command},
};

// Check that the .bcd data file exists and the shell startup script is setup
pub fn check_bookmarks_file(home: PathBuf) -> bool {
    let mut bookmarks_file = home;
    bookmarks_file.push(".bcd");
    println!("bookmarks-file: {}", bookmarks_file.display());
    bookmarks_file.exists()
}

// Attempt to setup your shell, can be run in interactive mode or not, and exits the process if cancelled unexpectedly.
pub fn setup_shell(interactive: bool) -> bool {
    let shell = ShellSetup::new();

    let home = if shell.is_in_snap {
        if let Some(home) = snap_data() {
            home
        } else {
            home_dir().unwrap()
        }
    } else {
        home_dir().unwrap()
    };
    let bookmarks_file_exists = check_bookmarks_file(home);
    if !bookmarks_file_exists {
        let mut bookmarks_file = home_dir().unwrap();
        bookmarks_file.push(".bcd");
        if !bookmarks_file.exists() {
            let _ = File::create(bookmarks_file);
        }
    }

    if !shell.init_setup {
        if interactive {
            if shell.is_supported {
                println!(
                    "\nIt looks like bookmark-cd (bcd) has not been set up to run in your shell `{}`.",
                    shell.name
                );
                if shell.is_in_snap {
                    println!(
                        "This may be because you have installed bcd from snap, which prevents automatic setup.\n"
                    );
                    if !shell.is_snap_connected {
                        println!("The snap container initially blocks access to shell init files that are needed to be checked for setup.  The following command can be run to unblock access to the required file and then try again:\n");
                        println!(
                            "    sudo snap connect bookmark-cd:{}\n \n",
                            shell.snap_connector
                        );
                    }
                    instructions_shell_script(shell.init, shell.eval);
                    false
                } else {
                    print!("Do you want to set this up now? [Y/n] ");
                    let _ = stdout().flush();
                    let mut reply = String::new();
                    let _b = std::io::stdin().read_line(&mut reply).unwrap();
                    reply = reply.trim().to_string();
                    if reply.eq_ignore_ascii_case("y")
                        || reply.eq_ignore_ascii_case("yes")
                        || reply.is_empty()
                    {
                        if shell.init.is_dir() {
                            println!(
                                "your shell `{}` is not currently supported",
                                shell.name.as_str()
                            );
                            false
                        } else {
                            setup_init_file(interactive, shell.init, shell.eval)
                        }
                    } else {
                        println!("Setup cancelled");
                        exit(1);
                    }
                }
            } else {
                if shell.name.as_str() == "unknown" {
                    println!("\nNo shell detected. Your environment is not currently supported.");
                } else {
                    println!(
                        "\nYour shell `{}` is not currently supported.",
                        shell.name.as_str()
                    );
                }
                false
            }
        } else {
            setup_init_file(interactive, shell.init, shell.eval)
        }
    } else {
        println!(
            "\nIt looks like bookmark-cd (bcd) has already been set up to run in your shell `{}`.",
            shell.name
        );
        true
    }
}

fn instructions_shell_script(init_file: PathBuf, eval: String) {
    println!("To complete setup, please edit your [{}] file and insert the following to the end of the file:\n", init_file.to_str().unwrap());
    println!("    # bookmark-cd init block");
    println!("    {eval}");
}

fn setup_init_file(_interactive: bool, init_file: PathBuf, eval: String) -> bool {
    // First, ensure the parent directory exists
    if let Some(parent) = init_file.parent() {
        match create_dir_all(parent) {
            Ok(_) => (),
            Err(error) => {
                println!("Failed to create directory structure: {error}");
                return false;
            }
        }
    }
    // Next, open the file for appending or create if it does not exist.
    let res = OpenOptions::new()
        .append(true)
        .create(true)
        .open(init_file.clone());
    match res {
        Ok(mut file) => {
            writeln!(file).unwrap();
            writeln!(file, "# bookmark-cd init block").unwrap();
            writeln!(file, "{eval}").unwrap();
            println!("\nYour shell startup script has been modified, restart your shell and type `bcd`\n");
            true
        }
        Err(x) => match x.kind() {
            ErrorKind::PermissionDenied => {
                println!(
                    "Shell startup script [{}] could not be created due to invalid permissions",
                    init_file.to_str().unwrap()
                );
                false
            }
            _ => {
                println!("Please run `bookmark-cd -i` to setup {x}");
                false
            }
        },
    }
}

#[allow(dead_code)]
pub fn initialise_completions_script(command: &mut clap::Command) {
    let (shell_name, _pid) = pshell::find().unwrap_or(("unknown".to_string(), 0));
    let generator = match shell_name.as_str() {
        "bash" => Some(clap_complete::Shell::Bash),
        "zsh" => {
            println!("autoload -Uz compinit && compinit");
            Some(clap_complete::Shell::Zsh)
        }
        "fish" => Some(clap_complete::Shell::Fish),
        "pwsh" => Some(clap_complete::Shell::PowerShell),
        "powershell" => Some(clap_complete::Shell::PowerShell),
        _ => None,
    };
    if let Some(shell) = generator {
        clap_complete::generate(shell, command, "bcd", &mut stdout());
    }
}

#[allow(dead_code)]
/// Outputs the shell script code for the function that will call this program, this should be used by an exec command
/// during shell initialisation, e.g. in .bashrc
pub fn initialise_shell_script() {
    let shell = ShellSetup::new();
    println!("{}", shell.init_cmd);
}

struct ShellSetup {
    name: String,
    init: PathBuf,
    init_setup: bool,
    eval: String,
    init_cmd: String,
    is_supported: bool,
    is_in_snap: bool,
    is_snap_connected: bool,
    snap_connector: String,
}

impl ShellSetup {
    fn new() -> ShellSetup {
        let (is_in_snap, home_override) = check_snap_home();
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
        let mut eval = String::new();
        let mut init_cmd: String = String::new();
        let is_supported = match shell_name.as_str() {
            "bash" => {
                eval =
                    "eval \"$(bookmark-cd init)\"\neval \"$(bookmark-cd completions)\"".to_string();
                init_cmd = include_str!("cmd_bash.sh").to_string();
                shell_init.push(".bashrc");
                if is_in_snap {
                    snap_connector = "dot-bashrc".to_string();
                }
                true
            }
            "fish" => {
                eval = "bookmark-cd init | source\nbookmark-cd completions | source".to_string();
                init_cmd = include_str!("cmd_fish.sh").to_string();
                shell_init.push(".config/fish/config.fish");
                if is_in_snap {
                    snap_connector = "dot-config-fish".to_string();
                }
                true
            }
            "ksh" => {
                eval = "bookmark-cd init > ~/.bcd_ksh\n. ~/.bcd_ksh".to_string();
                init_cmd = include_str!("cmd_ksh.sh").to_string();
                shell_init.push(".kshrc");
                if is_in_snap {
                    snap_connector = "dot-kshrc".to_string();
                }
                true
            }
            "zsh" => {
                eval =
                    "eval \"$(bookmark-cd init)\"\neval \"$(bookmark-cd completions)\"".to_string();
                init_cmd = include_str!("cmd_bash.sh").to_string();
                shell_init.push(".zshrc");
                if is_in_snap {
                    snap_connector = "dot-zshrc".to_string();
                }
                true
            }
            "pwsh" => {
                eval = "bookmark-cd init | Out-String | Invoke-Expression\nbookmark-cd completions | Out-String | Invoke-Expression".to_string();
                init_cmd = include_str!("cmd_pwsh.ps1").to_string();
                let profile_output = Command::new("pwsh")
                    .args(["-NoProfile", "-Command", "echo", "$PROFILE"])
                    .output();
                match profile_output {
                    Ok(profile_output) => {
                        if let Ok(profile_path) = String::from_utf8(profile_output.stdout) {
                            shell_init.push(profile_path.trim());
                        }
                    }
                    Err(x) => {
                        println!("Failed to get powershell $PROFILE: {x}");
                    }
                };
                true
            }
            "powershell" => {
                eval = "bookmark-cd init | Out-String | Invoke-Expression\nbookmark-cd completions | Out-String | Invoke-Expression".to_string();
                init_cmd = include_str!("cmd_pwsh.ps1").to_string();
                let profile_output = Command::new("powershell")
                    .args(["-NoProfile", "-Command", "echo", "$PROFILE"])
                    .output();
                match profile_output {
                    Ok(profile_output) => {
                        if let Ok(profile_path) = String::from_utf8(profile_output.stdout) {
                            shell_init.push(profile_path.trim());
                        }
                    }
                    Err(x) => {
                        println!("Failed to get powershell $PROFILE: {x}");
                    }
                };
                true
            }
            _ => false,
        };
        let shell_init_exists = shell_init.exists();
        let init_setup = if shell_init_exists {
            let file_res = File::open(shell_init.clone());
            if let Ok(mut file) = file_res {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => contents.contains(&eval) && !eval.is_empty(),
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
            init_setup,
            eval,
            init_cmd,
            is_in_snap,
            is_supported,
            is_snap_connected,
            snap_connector,
        }
    }
}
