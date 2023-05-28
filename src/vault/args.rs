// Use
use clap::CommandFactory;
use clap::Parser;
use std::path::PathBuf;

/// Args struct
#[derive(Debug, Default, Parser)]
#[command(about, long_about = None)]
#[command(help_template = "{about-section}Version: {version}\nAuthor: {author}\n\n{usage-heading} {usage}\n\n{all-args}{tab}")]
#[command(author)]
#[command(version)]
pub struct Args
{
	/// Config file
	///
	/// Empty = Show help.
	#[arg(short, long, value_name = "FILE")]
	pub config: Option<PathBuf>,

	/// Debug flag
	///
	/// Display debug data.
	#[arg(short, long, default_value_t  = false)]
	pub debug: bool,

	/// Output sample configuration file.
	///
	/// This will output a sample configuration file.
	/// It will describe all configuration properties.
	/// The configuration format is TOML (Tom's Obvious Minimal Language).
	#[arg(short, long, default_value_t = false)]
	pub sample: bool,

	/// Task to be executed
	///
	/// Empty = Do nothing.
	/// * = Do all tasks.
	#[arg(short, long, value_name = "TASK")]
	pub task: Option<String>,
}

/// Args impl
impl Args
{
	/// Cmd
	pub fn cmd() -> clap::Command
	{
		return Args::command();
	}

	/// Read
	pub fn read() -> Args
	{
		return Args::parse();
	}
}
