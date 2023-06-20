use clap::Parser;
use log::LevelFilter;

use crate::args::Args;

use pwg::http;
use pwg::{generate_password, Charset, DEFAULT_CHARSET};

mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    let args = Args::parse();

    if args.http {
        http::start(args.port).await?;
    } else {
        println!(
            "{}",
            match args.charset {
                Some(charset) => generate_password(args.length, &Charset::from(&charset)),
                None => generate_password(args.length, &DEFAULT_CHARSET),
            }
        )
    }

    Ok(())
}
