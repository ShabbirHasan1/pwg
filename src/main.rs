use clap::Parser;
use log::LevelFilter;

use crate::args::Args;
use crate::gen::{password, Charset, DEFAULT_CHARSET};

mod args;
mod gen;
mod http;

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
                Some(charset) => password(args.length, &Charset::from(&charset)),
                None => password(args.length, &DEFAULT_CHARSET),
            }
        )
    }

    Ok(())
}
