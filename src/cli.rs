use clap::Parser;

/// Struct containing the parsed command line arguments
#[derive(Parser)]
#[command(name = "bcd")]
#[command(bin_name = "bcd")]
#[command(author, version, about, long_about = None, arg_required_else_help(true), disable_version_flag(true))]
pub struct Options {
    /// Bookmarked directory to change to
    #[arg(value_parser)]
    pub bookmark: Option<String>,

    /// Setup the the shell startup script
    #[arg(short, long, value_parser, display_order(0), hide(true))]
    pub install: bool,

    /// Store the current directory as a bookmark
    #[arg(short, long, value_parser, display_order(1))]
    pub store: Option<String>,

    /// Remove a specified bookmark
    #[arg(short, long, value_parser, display_order(2))]
    pub remove: Option<String>,

    /// List the stored bookmarks
    #[arg(short, long, value_parser, display_order(3))]
    pub list: bool,

    /// Print version information
    #[arg(short = 'V', long, value_parser, display_order(4))]
    pub version: bool,
}
