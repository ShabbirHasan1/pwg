//! # An HTTP API for the password generator
//!
//! It is a pretty straightforward API that uses the [`gen`](../gen/mod.rs) module to generate
//! passwords.
//!
//! ## Starting
//!
//! To start the server, you can run the `pwg` CLI tool with the `--http` option like so:
//!
//! ```bash
//! $ pwg --http
//! ```
//!
//! HTTP options are specified in [`args.rs`](../args.rs).
//!
//! You may also start the server programmatically by calling `start()` with the appropriate
//! arguments.
//!
//! ## Endpoints
//!
//! ### `GET /`
//! Returns a random password with a length of 16 characters by default. The length can be changed
//! by passing a `length` query parameter like so: `GET /?length=32`.

use std::{net::SocketAddr, time::Instant};

use axum::{extract::Query, routing::get, Router};
use log::info;
use serde::Deserialize;

use crate::gen::{password, Charset, DEFAULT_CHARSET};

/// The query parameters for the `GET /` endpoint.
#[derive(Deserialize, Debug)]
struct GeneratePasswordQuery {
    length: Option<usize>,
    charset: Option<String>,
}

/// Generate a password.
async fn generate_password(Query(params): Query<GeneratePasswordQuery>) -> String {
    let start_time = Instant::now();

    let length = params.length.unwrap_or(16);
    let password = match &params.charset {
        Some(charset) => password(length, &Charset::from(charset)),
        None => password(length, &DEFAULT_CHARSET),
    };

    info!(
        "Processed {:?} in {}ms",
        params,
        start_time.elapsed().as_millis()
    );

    password
}

/// Start the HTTP server.
pub async fn start(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new().route("/", get(generate_password));

    info!("Starting HTTP server on http://127.0.0.1:{}", port);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
