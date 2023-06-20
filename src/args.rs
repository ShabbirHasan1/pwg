//! The command-line argument definitions.
//!
//! It is one Clap parser, so you can simply perform `Args::parse()` to parse the arguments.

use clap::Parser;

/// A password generator.
#[derive(Parser)]
pub struct Args {
    /// Whether to start the HTTP interface.
    #[arg(long, default_value = "false")]
    pub http: bool,
    /// The port for the HTTP server (if enabled).
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// The length of the password.
    #[arg(short, long, default_value = "16")]
    pub length: usize,
    /// The charset to use.
    ///
    /// `DEFAULT_CHARSET` is used if this is `None`.
    #[arg(short, long)]
    pub charset: Option<String>,
}
