#![deny(missing_docs, unused_crate_dependencies)]

//! Atomicals mining manager.

mod cli;
use cli::Cli;

mod electrumx;
mod engine;
mod util;
mod wallet;

mod prelude {
	pub use anyhow::{Error, Result};
}
use prelude::*;

// crates.io
use clap::Parser;
use tracing::{Level};

#[tokio::main]
async fn main() -> Result<()> {
	color_eyre::install().unwrap();
	let file_appender = tracing_appender::rolling::daily(".", "log");
	let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

	tracing_subscriber::fmt().with_max_level(Level::INFO)
		.with_writer(non_blocking).with_target(false).init();

	Cli::parse().run().await?;

	Ok(())
}
