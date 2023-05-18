/// Main
fn main()
{
	run();
}

// Usage
use std::path::{Path, PathBuf};
use clap::{CommandFactory, Parser};

/// Args struct
///
/// Holding all program arguments.
#[derive(Debug, Default, clap::Parser)]
#[command(about, long_about = None)]
#[command(help_template = "{about-section}Version: {version}\nAuthor: {author}\n\n{usage-heading} {usage}\n\n{all-args}{tab}")]
#[command(author)]
#[command(version)]
struct Args
{
	/// Config file
	///
	/// Empty = Show help
	#[arg(short, long, value_name = "FILE")]
	config: Option<PathBuf>,

	/// Debug flag
	///
	/// Shows additional debug information.
	#[arg(short, long, default_value_t = false)]
	debug: bool,

	/// Task to be executed
	///
	/// Empty = Do nothing.
	/// * = Do all tasks.
	#[arg(short, long, value_name = "TASK")]
	task: Option<String>,
}

/// Get path of program function
///
/// Gets program path without executable filename.
fn get_path_program() -> PathBuf
{
	match std::env::current_exe()
	{
		Ok(path) =>
		{
			match path.parent()
			{
				Some(parent) => parent.to_path_buf(),
				_ => PathBuf::from("."),
			}
		},
		_ => PathBuf::from("."),
	}
}

/// Print the help
///
/// Prints the help text to the console.
fn print_help(long: bool)
{
	if long
	{
		match Args::command().print_long_help()
		{
			Ok(_) => {},
			Err(_) => println!("Error: Failed to show long help text!"),
		}
	}
	else
	{
		match Args::command().print_help()
		{
			Ok(_) => {},
			Err(_) => println!("Error: Failed to show short help text!"),
		}

	}
}

/// Run
///
/// Run the program.
fn run() -> bool
{
	// Get arguments
	let args = Args::parse();

	// Debug output
	if args.debug
	{
		println!("Debug");
		println!("{{");
		println!("    args: {:?},", args);
		println!("    path_program: {:?},", get_path_program());
		println!("}}\n");
	}

	// Config given
	if let Some(config) = args.config.as_deref()
	{
		// Task given
		if let Some(task) = args.task.as_deref()
		{
			// All tasks
			if task.eq("*")
			{
				run_tasks(config)
			}

			// One specific task
			else
			{
				run_task(config, task)
			}
		}

		// No task given
		else
		{
			println!("Error: No task specified!\n");
			print_help(false);
			false
		}
	}

	// No config given
	else
	{
		println!("Error: No configuration specified!\n");
		print_help(false);
		false
	}
}

/// Run task
///
/// Run a task from config.
fn run_task(config: &Path, task: &str) -> bool
{
	// Hail
	println!("Running task '{}' from config '{}'...", task, config.display());

	// TODO: Run the task.
	false
}

/// Run tasks
///
/// Run all tasks from config.
fn run_tasks(config: &Path) -> bool
{
	// Hail
	println!("Running all tasks from config '{}'...", config.display());

	// TODO: Iterate over all tasks and call run_task.
	false
}
