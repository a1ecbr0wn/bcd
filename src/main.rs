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
    let options = cli::Options::parse();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        exit(0);
    } else {
        if args.contains(&("init".to_string())) {
            init::initialise_shell_script();
            exit(0);
        }

        lazy_static! {
            static ref DESCRIPTION: String = format!(
                "bcd {}: bookmark directories and move to them.",
                env!("CARGO_PKG_VERSION")
            );
        };

        if options.show_version {
            println!("{}", DESCRIPTION.as_str());
            exit(0);
        }

        if options.install {
            init::setup_shell();
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
                        bookmarks_cache.insert(record[0].to_string(), record[1].to_string());
                    }
                }
                _ => println!("Directory bookmarks file could not be read"),
            }
        } else {
            println!("Directory bookmarks file not found.");
            init::setup_shell();
            exit(0);
        }

        if options.store.is_some() {
            let path = current_dir().unwrap();
            let _rtn = bookmarks_cache.insert(
                options.store.unwrap(),
                path.into_os_string().into_string().unwrap(),
            );
            if persist(&bookmarks_cache, bookmarks_file.as_path()).is_ok() {
                println!("Bookmark saved");
            }
            exit(0);
        }

        if options.remove.is_some() {
            let to_remove = options.remove.unwrap();
            let removed = bookmarks_cache.remove(&to_remove);
            if removed.is_some() && persist(&bookmarks_cache, bookmarks_file.as_path()).is_ok() {
                println!("Bookmark removed");
            } else {
                println!("{} is not a valid bookmark", &to_remove);
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

        if options.bookmark.is_some() {
            match bookmarks_cache.get(options.bookmark.unwrap().as_str()) {
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
    wtr.write_record(&["bookmark", "path"])?;
    for bookmark in bookmarks.iter() {
        wtr.write_record(&[bookmark.0, bookmark.1])?;
    }
    wtr.flush()?;
    Ok(())
}
