use clap::Parser;

/// Struct containing the parsed command line arguments
#[derive(Parser)]
#[clap(name = "bcd")]
#[clap(bin_name = "bcd")]
#[clap(author, version, about, long_about = None)]
pub struct Options {
    /// Print version information
    #[clap(short = 'V', long = "version", value_parser)]
    pub show_version: bool,

    /// Check the bcd shell function is setup correctly
    #[clap(short, long, value_parser)]
    pub install: bool,

    /// List the bookmarks
    #[clap(short, long, value_parser)]
    pub list: bool,

    /// Store the current directory as a bookmark STORE
    #[clap(short, long, value_parser)]
    pub store: Option<String>,

    /// Bookmarked directory to change to
    #[clap(value_parser)]
    pub bookmark: Option<String>,
}
