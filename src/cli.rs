use clap::Parser;

/// Struct containing the parsed command line arguments
#[derive(Parser)]
#[clap(name = "bcd")]
#[clap(bin_name = "bcd")]
#[clap(author, version, about, long_about = None, arg_required_else_help(true), disable_version_flag(true))]
pub struct Options {
    /// Print version information
    #[clap(short = 'V', long, value_parser)]
    pub version: bool,

    /// List the bookmarks
    #[clap(short, long, value_parser)]
    pub list: bool,

    /// Store the current directory as a bookmark STORE
    #[clap(short, long, value_parser)]
    pub store: Option<String>,

    /// Remove a specified bookmark REMOVE
    #[clap(short, long, value_parser)]
    pub remove: Option<String>,

    /// Bookmarked directory to change to
    #[clap(value_parser)]
    pub bookmark: Option<String>,
}
