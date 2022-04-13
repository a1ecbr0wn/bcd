use argparse::{ArgumentParser, Store, StoreTrue};
use csv::{Reader, Result, Writer};
use home::home_dir;
use lazy_static::lazy_static;
use std::env;
use std::io::stdout;
use std::path::Path;
use std::process::exit;
use std::{collections::BTreeMap, env::current_dir};
use tabled::{builder::Builder, Alignment, Full, Modify, Style};

mod init;

/// Struct containing the parsed command line arguments
struct Options {
    show_version: bool,
    install: bool,
    list: bool,
    store: String,
    target: String,
}

// Parse the command line arguments
fn parse_arguments(description: &str, options: &mut Options) {
    let mut parser = ArgumentParser::new();
    parser.set_description(description);
    parser.refer(&mut options.install).add_option(
        &["-i", "--install"],
        StoreTrue,
        "Check the bcd shell function is setup correctly",
    );
    parser.refer(&mut options.show_version).add_option(
        &["-V", "--version"],
        StoreTrue,
        "Get version info",
    );
    parser
        .refer(&mut options.list)
        .add_option(&["-l", "--list"], StoreTrue, "List the bookmarks");
    parser.refer(&mut options.store).add_option(
        &["-s", "--store"],
        Store,
        "Store the current directory as a bookmark STORE",
    );
    parser.refer(&mut options.target).add_argument(
        "BOOKMARK",
        Store,
        "Bookmarked directory to change to",
    );
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        parser.print_help("bcd", &mut stdout()).unwrap();
        exit(0);
    } else {
        if args.contains(&("init".to_string())) {
            init::initialise_shell_script();
            exit(0);
        }
        match parser.parse_args() {
            Ok(()) => {}
            Err(x) => {
                exit(x);
            }
        }
    }
}

fn main() {
    let mut options = Options {
        show_version: false,
        install: false,
        list: false,
        store: "".to_string(),
        target: "".to_string(),
    };

    lazy_static! {
        static ref DESCRIPTION: String = format!(
            "bcd v{}: bookmark directories and move to them.",
            env!("CARGO_PKG_VERSION")
        );
    };

    parse_arguments(DESCRIPTION.as_str(), &mut options);

    if options.show_version {
        println!("{}", DESCRIPTION.as_str());
    }

    if options.install {
        init::setup_shell();
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

    if !options.store.is_empty() {
        let path = current_dir().unwrap();
        bookmarks_cache.insert(options.store, path.into_os_string().into_string().unwrap());
        if persist(&bookmarks_cache, bookmarks_file.as_path()).is_ok() {
            println!("Bookmark saved");
        }
    }

    if options.list {
        if !bookmarks_cache.is_empty() {
            let mut table = Builder::default().set_header(["bookmark", "path"]);
            for bookmark in bookmarks_cache.clone() {
                table = table.add_row([bookmark.0, bookmark.1]);
            }
            println!(
                "{}",
                table
                    .build()
                    .with(Style::psql())
                    .with(Modify::new(Full).with(Alignment::left()))
            );
        } else {
            println!("Use the following arguments to store your first bookmark:");
            println!("  -s,--store STORE      Store the current directory as a bookmark STORE");
        }
    }

    if !options.target.is_empty() {
        match bookmarks_cache.get(options.target.as_str()) {
            Some(path) => {
                println!("cd {}", path);
            }
            _ => {
                println!("Bookmark not found, use `--list` to see the list of bookmarks");
            }
        }
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
