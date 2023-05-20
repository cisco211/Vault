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
	/// Empty = Show help
	#[arg(short, long, value_name = "FILE")]
	pub config: Option<PathBuf>,

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
