// Mod
mod args;
mod config;
mod util;

/// Run help
///
/// Prints the help text to the console.
fn help(long: bool)
{
	if long
	{
		match args::command().print_long_help()
		{
			Ok(_) => {},
			Err(_) => println!("Error: Failed to show long help text!"),
		}
	}
	else
	{
		match args::command().print_help()
		{
			Ok(_) => {},
			Err(_) => println!("Error: Failed to show short help text!"),
		}

	}
}

/// Run
///
/// Run the program.
pub fn run() -> bool
{
	// Get arguments
	let args = args::parse();

	// Debug output
	if args.debug
	{
		println!("Debug");
		println!("{{");
		println!("    args: {:?},", args);
		println!("    path_program: {:?},", util::get_path_program());
		println!("}}\n");
	}

	// Config given
	if let Some(c) = args.config.as_deref()
	{
		// Task given
		if let Some(t) = args.task.as_deref()
		{
			// All tasks
			if t.eq("*")
			{
				tasks(c)
			}

			// One specific task
			else
			{
				task(c, t)
			}
		}

		// No task given
		else
		{
			println!("Error: No task specified!\n");
			help(false);
			false
		}
	}

	// No config given
	else
	{
		println!("Error: No configuration specified!\n");
		help(false);
		false
	}
}

/// Run task
///
/// Run a task from config.
fn task(config: &std::path::Path, task: &str) -> bool
{
	// Hail
	println!("Running task '{}' from config '{}'...", task, config.display());

	// TODO: Run the task.
	false
}

/// Run tasks
///
/// Run all tasks from config.
fn tasks(config: &std::path::Path) -> bool
{
	// Hail
	println!("Running all tasks from config '{}'...", config.display());

	// TODO: Iterate over all tasks and call run_task.
	false
}
