/// Data struct
///
/// Holding all program arguments.
#[derive(Debug, Default, clap::Parser)]
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
	pub config: Option<std::path::PathBuf>,

	/// Debug flag
	///
	/// Shows additional debug information.
	#[arg(short, long, default_value_t = false)]
	pub debug: bool,

	/// Task to be executed
	///
	/// Empty = Do nothing.
	/// * = Do all tasks.
	#[arg(short, long, value_name = "TASK")]
	pub task: Option<String>,
}
