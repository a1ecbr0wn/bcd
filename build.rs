use clap::CommandFactory;
use std::{fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");

    #[path = "src/init/mod.rs"]
    mod init;

    init::setup_shell(false);

    #[path = "src/cli.rs"]
    mod cli;

    let out_dir = &PathBuf::from("target/.fig");
    let _ = fs::create_dir_all(out_dir);

    clap_complete::generate_to(
        clap_complete_fig::Fig,
        &mut cli::Options::command(),
        "bcd",
        out_dir,
    )
    .expect("Unable to generate Fig spec");
}
