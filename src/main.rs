use clap::Parser;
use csv::{Reader, Result, Writer};
use home::home_dir;
use lazy_static::lazy_static;
use std::env;
use std::path::Path;
use std::process::exit;
use std::{collections::BTreeMap, env::current_dir};
use tabled::{builder::Builder, object::Segment, Alignment, Modify, Style};

mod cli;
mod init;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // If you are running this for the first time, set up your shell
        if init::setup_shell(false) {
            let _options = cli::Options::parse();
            exit(0);
        } else {
            exit(1);
        }
    } else {
        if args.contains(&("init".to_string())) {
            // Not called directly, but called by the shell function `bcd` set up in the shell startup script
            init::initialise_shell_script();
            exit(0);
        }

        let options = cli::Options::parse();

        lazy_static! {
            static ref DESCRIPTION: String = format!(
                "bcd {}: bookmark directories and move to them.",
                env!("CARGO_PKG_VERSION")
            );
        };

        if options.install {
            // a way to try to set up the shell  when the data file exists but the `bcd` function is not.
            if init::setup_shell(true) {
                exit(0);
            } else {
                exit(1);
            }
        }

        if options.version {
            println!("{}", DESCRIPTION.as_str());
            exit(0);
        }

        let mut bookmarks_cache: BTreeMap<String, String> = BTreeMap::new();
        let mut bookmarks_file = home_dir().unwrap();
        bookmarks_file.push(".bcd");
        if bookmarks_file.exists() {
            let res = Reader::from_path(bookmarks_file.as_path());
            match res {
                Ok(mut res) => {
                    for result in res.records() {
                        let record = result.expect("a CSV record");
                        if record.len() >= 2 {
                            bookmarks_cache.insert(record[0].to_string(), record[1].to_string());
                        } else {
                            println!(
                                "Reading file `{}`, skipping `{}`",
                                bookmarks_file.display(),
                                record.as_slice()
                            );
                        }
                    }
                }
                _ => println!("Directory bookmarks file could not be read"),
            }
        } else {
            println!("Directory bookmarks file not found.");
            if init::setup_shell(true) {
                exit(0);
            } else {
                exit(1);
            }
        }

        if let Some(key) = options.store {
            if key.len() < 50 {
                let path = current_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap();
                if let Some(updated) = bookmarks_cache.insert(key.clone(), path.clone()) {
                    if persist(&bookmarks_cache, bookmarks_file.as_path()).is_ok() {
                        println!("Bookmark `{key}`: `{path}` updated from `{key}`: `{updated}`");
                    } else {
                        println!(
                            "Failed to update `{key}` bookmark, bookmark file is not writable"
                        );
                    }
                } else if persist(&bookmarks_cache, bookmarks_file.as_path()).is_ok() {
                    println!("Bookmark `{key}`: `{path}` saved");
                } else {
                    println!("Failed to add `{key}` bookmark, bookmark file is not writable");
                }
            } else {
                println!("Bookmark names cannot be more than 50 characters long `{key}`")
            }
            exit(0);
        }

        if let Some(key) = options.remove {
            if let Some(removed) = bookmarks_cache.remove(&key) {
                if persist(&bookmarks_cache, bookmarks_file.as_path()).is_ok() {
                    println!("Bookmark `{key}`: `{removed}` removed");
                } else {
                    println!("Failed to remove `{key}` bookmark, bookmark file is not writable");
                }
            } else {
                println!("`{}` is not a valid bookmark", &key);
            }
            exit(0);
        }

        if options.list {
            if !bookmarks_cache.is_empty() {
                let mut builder = Builder::default();
                builder.set_columns(["bookmark", "path"]);
                for bookmark in bookmarks_cache.clone() {
                    builder.add_record([bookmark.0, bookmark.1]);
                }
                println!(
                    "{}",
                    builder
                        .build()
                        .with(Style::psql())
                        .with(Modify::new(Segment::all()).with(Alignment::left()))
                );
            } else {
                println!("Use the following arguments to store your first bookmark:");
                println!("  -s,--store STORE      Store the current directory as a bookmark STORE");
            }
            exit(0);
        }

        if let Some(key) = options.bookmark {
            match bookmarks_cache.get(key.as_str()) {
                Some(path) => {
                    println!("cd {}", path);
                }
                _ => {
                    println!("Bookmark not found, use `--list` to see the list of bookmarks");
                }
            }
            exit(0);
        }

        println!("{}", DESCRIPTION.as_str());
    }
}

fn persist(bookmarks: &BTreeMap<String, String>, path: &Path) -> Result<()> {
    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(["bookmark", "path"])?;
    for bookmark in bookmarks.iter() {
        wtr.write_record([bookmark.0, bookmark.1])?;
    }
    wtr.flush()?;
    Ok(())
}
