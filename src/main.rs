use clap::Parser;
use log::LevelFilter;

use crate::args::Args;

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
        println!("{}", gen::password(args.length, &gen::DEFAULT_CHARSET));
    }

    Ok(())
}
