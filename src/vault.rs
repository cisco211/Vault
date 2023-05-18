// Mod
mod args;
mod config;
mod util;

/// Do command
fn do_command(cfg: &config::Config, task: &str) -> bool
{
	// Get task
	let t = match config::task_get(cfg, task)
	{
		Some(t_) => t_,
		None => return false,
	};

	// Get args array
	let a = match t["args"].as_array()
	{
		Some(c_) => c_,
		None => return false,
	};

	// Get cmd string
	let c = match t["cmd"].as_str()
	{
		Some(c_) => c_,
		None => return false,
	};

	// Get path string
	let p = match t["path"].as_str()
	{
		Some(p_) => p_,
		None => return false,
	};

	// Create command
	let mut cmd = std::process::Command::new(c);

	// Set working directory
	cmd.current_dir(p);

	// Create now string
	let n = util::time_to_string(util::time_now());

	// Add arguments
	for v in a
	{
		match v.as_str()
		{
			Some(v_) => cmd.arg(v_.replace("{NOW}", n.as_str())),
			None => continue,
		};
	}

	// Execute command
	match cmd.status()
	{
		Ok(s_) => return s_.success(),
		Err(e) =>
		{
			println!("Error: Task '{}' failed to execute command '{}'!\n{}", task, c, e.to_string());
			return false;
		}
	};
}

/// Do prepare
fn do_prepare(cfg: &config::Config, task: &str) -> bool
{
	// Get task
	let t = match config::task_get(cfg, task)
	{
		Some(t_) => t_,
		None => return false,
	};

	// Get path string
	let p = match t["path"].as_str()
	{
		Some(p_) => p_,
		None => return false,
	};

	// Get path buffer
	let path = std::path::PathBuf::new().join(p);

	// Create path recursively
	match std::fs::create_dir_all(path.to_path_buf())
	{
		Ok(_) => (),
		Err(e) =>
		{
			println!("Error: Task '{}' failed to create path path '{}'!\n{}", task, path.display(), e.to_string());
			return false;
		}
	}

	// Done
	return true;
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
		println!("    prwd: {:?},", util::path_program());
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
				return tasks(&cfg);
			}

			// One specific task
			else
			{
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
	// Hail
	println!("Running configuration '{}' task '{}'...", cfg.name, task);

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

	// Do prepare
	if !do_prepare(cfg, task)
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
	return true;
}

/// Run tasks
///
/// Run all tasks from config.
fn tasks(cfg: &config::Config) -> bool
{
	// Hail
	println!("Running configuration '{}' all tasks...", cfg.name);

	// Status bool
	let mut b = true;

	// Iterate over tasks
	for k in cfg.tasks.keys()
	{
		// Run task
		if !task(cfg, k.as_str())
		{
			b = false;
		}
	}

	// Done
	return b;
}
