// Mod
mod args;
mod config;
mod util;

/// Do command
fn do_command(_cfg: &config::Config, _task: &str) -> bool
{
	// TODO: Implement do_command.
	true
}

/// Do rotation
fn do_rotation(_cfg: &config::Config, _task: &str) -> bool
{
	// TODO: Implement do_rotation.
	true
}

/// Run help
///
/// Prints the help text to the console.
fn help(long: bool)
{
	println!("");
	if long
	{
		match args::command().print_long_help()
		{
			Ok(_) => (),
			Err(_) => println!("Error: Failed to show long help text!"),
		}
	}
	else
	{
		match args::command().print_help()
		{
			Ok(_) => (),
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
	if let Some(arg_config) = args.config.as_deref()
	{
		// Load configuration
		let cfg = match config::load(arg_config)
		{
			Some(c) => c,
			None => return false,
		};

		// No name
		if cfg.name.is_empty()
		{
			println!("Error: Configuration file '{}' has no name!", arg_config.display());
			return false;
		}

		// Task given
		if let Some(arg_task) = args.task.as_deref()
		{
			// Empty task
			if arg_task.is_empty()
			{
				println!("Error: Task is empty!");
				return false;
			}

			// All tasks
			if arg_task.eq("*")
			{
				println!("Running all tasks from configuration '{}'...", cfg.name);
				return tasks(&cfg);
			}

			// One specific task
			else
			{
				println!("Running task '{}' from configuration '{}'...", arg_task, cfg.name);
				return task(&cfg, arg_task);
			}
		}

		// No task given
		else
		{
			println!("Error: No task specified!");
			help(false);
			return false;
		}
	}

	// No config given
	else
	{
		println!("Error: No configuration file specified!");
		help(false);
		return false;
	}
}

/// Run task
///
/// Run a task from config.
fn task(cfg: &config::Config, task: &str) -> bool
{
	// Debug
	println!("Debug: vault::task\n    cfg = {:?}\n    task = {:?}", cfg, task);

	// Get task
	let t = match config::task_get(cfg, task)
	{
		Some(t_) => t_,
		None => return false,
	};

	// Task is not valid
	if !config::task_valid(task, &t)
	{
		return false;
	}

	// Do rotation
	if !do_rotation(cfg, task)
	{
		return false;
	}

	// Do command
	if !do_command(cfg, task)
	{
		return false;
	}

	// Done
	println!("Debug: vault::task done!");
	return true;
}

/// Run tasks
///
/// Run all tasks from config.
fn tasks(cfg: &config::Config) -> bool
{
	// Debug
	println!("Debug: vault::tasks\n    cfg = {:?}", cfg);

	// TODO: Iterate over all tasks and call run_task.
	println!("Debug: vault::tasks done!");
	return true;
}
