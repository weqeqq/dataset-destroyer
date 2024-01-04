#![allow(dead_code)]
use anyhow::Result;
use anyhow::anyhow;
use config::Config;
use console::Term;

mod processor;
mod config;

fn main() -> Result<()> {
	let term = Term::stdout();
	term.hide_cursor()?;
	term.clear_screen()?;
	term.set_title("processing...");

	if !term.is_term() {
		return Err(anyhow!("Open in the terminal"));
	}

	let config = Config::open("default.yaml")?;
	config.start_processing()?;

	Ok(())
}
