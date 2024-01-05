#![allow(dead_code)]
use anyhow::Result;
use anyhow::anyhow;
use config::Config;
use console::Term;

use clap::Parser;
use std::path::PathBuf;

mod processor;
mod config;

#[derive(Parser)]
#[command(name = "Dataset destroyer")]
#[command(author = "Weqeq")]
#[command(version, about, long_about = None)]
struct Cli {
	/// Path to your configuration file
	#[arg(long, value_name = "FILE")]
	config: Option<PathBuf>,
}

fn main() -> Result<()> {
	let term = Term::stdout();
	term.hide_cursor()?;
	term.clear_screen()?;
	term.set_title("processing...");

	if !term.is_term() {
		return Err(anyhow!("Open in the terminal"));
	}

	let cli = Cli::parse();

	let config = if let Some(path) = cli.config.as_deref() {
		path
	} else {
		return Err(anyhow!("config"));
	};

	let config = Config::open(config)?;
	config.start_parallel_processing()?;

	Ok(())
}
