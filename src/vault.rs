// Mod
mod args;
mod config;
mod task;
mod util;

/// Do command
fn do_command(a_cfg: &config::Config, a_task: &str) -> bool
{
	// Hail
	println!("{}.{} executing...", a_cfg.name, a_task);

	// Get task
	let l_task_c = match config::task_get(a_cfg, a_task)
	{
		Some(m_task) => m_task,
		None => return false,
	};

	// Get args array
	let l_task_args = match l_task_c["args"].as_array()
	{
		Some(m_value) => m_value,
		None => return false,
	};

	// Get cmd string
	let l_task_cmd = match l_task_c["cmd"].as_str()
	{
		Some(m_value) => m_value,
		None => return false,
	};

	// Get path string
	let l_task_path = match l_task_c["path"].as_str()
	{
		Some(m_value) => m_value,
		None => return false,
	};

	// Get interval integer
	let l_task_interval = match l_task_c["interval"].as_integer()
	{
		Some(m_value) => m_value,
		None => return false,
	};

	// Create command
	let mut l_cmd = std::process::Command::new(l_task_cmd);

	// Set working directory
	l_cmd.current_dir(l_task_path);

	// Now
	let l_now = util::time_now();

	// Add arguments
	for i_value in l_task_args
	{
		match i_value.as_str()
		{
			Some(m_value) => l_cmd.arg(m_value.replace("{NOW}", util::time_to_string(l_now).as_str())),
			None => continue,
		};
	}

	// Execute command
	let l_status = match l_cmd.status()
	{
		Ok(m_status) => m_status,
		Err(m_error) =>
		{
			println!("Error: Task '{}' failed to execute command '{}'!\n{}", a_task, l_task_cmd, m_error.to_string());
			return false;
		}
	};

	// Execution failed
	if !l_status.success()
	{
		println!("Error: Task '{}' failed to execute command '{}'!", a_task, l_task_cmd);
		return false;
	}

	// Get task file path
	let l_path = std::path::PathBuf::new().join(l_task_path);

	// Load task
	let mut l_task_t = match task::load(&l_path)
	{
		Some(m_task) => m_task,
		None => return false,
	};

	// Update expiration date
	l_task_t.expires = util::time_to_string(l_now + chrono::Duration::seconds(l_task_interval));

	// Save task
	if !task::save(&l_path, &l_task_t)
	{
		return false;
	}

	// Done
	println!("{}.{} done (next: {}).\n", a_cfg.name, a_task, l_task_t.expires);
	return true;
}

/// Do prepare
fn do_prepare(a_cfg: &config::Config, a_task: &str) -> bool
{
	// Hail
	println!("{}.{} preparing...", a_cfg.name, a_task);

	// Get config task
	let l_task_c = match config::task_get(a_cfg, a_task)
	{
		Some(m_task) => m_task,
		None => return false,
	};

	// Get path string
	let l_path = match l_task_c["path"].as_str()
	{
		Some(m_path) => std::path::PathBuf::new().join(m_path),
		None => return false,
	};

	// Create task if not exist
	if !task::create(&l_path)
	{
		return false;
	}

	// Load task
	let l_task_t = match task::load(&l_path)
	{
		Some(m_task) => m_task,
		None => return false,
	};

	// Get expiration date
	let l_expires = match util::time_from_string(l_task_t.expires.as_str())
	{
		Some(m_expires) => m_expires,
		None => return false,
	};

	// Not yet expired
	if util::time_now() < l_expires
	{
		println!("{}.{} skipped (expires: {}).\n", a_cfg.name, a_task, l_task_t.expires);
		return false;
	}

	// Done
	return true;
}

/// Do rotation
fn do_rotation(a_cfg: &config::Config, a_task: &str) -> bool
{
	// Hail
	println!("{}.{} rotating...", a_cfg.name, a_task);

	// TODO: Implement do_rotation.
	true
}

/// Run help
///
/// Prints the help text to the console.
fn help(a_long: bool)
{
	println!("");
	if a_long
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
	let l_args = args::parse();

	// Config given
	if let Some(l_config) = l_args.config.as_deref()
	{
		// Load configuration
		let l_cfg = match config::load(l_config)
		{
			Some(m_cfg) => m_cfg,
			None => return false,
		};

		// No name
		if l_cfg.name.is_empty()
		{
			println!("Error: Configuration file '{}' has no name!", l_config.display());
			return false;
		}

		// Task given
		if let Some(l_task) = l_args.task.as_deref()
		{
			// Empty task
			if l_task.is_empty()
			{
				println!("Error: Task is empty!");
				return false;
			}

			// All tasks
			if l_task.eq("*")
			{
				return tasks(&l_cfg);
			}

			// One specific task
			else
			{
				return task(&l_cfg, l_task);
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
fn task(a_cfg: &config::Config, a_task: &str) -> bool
{
	// Hail
	println!("{}.{} checking...", a_cfg.name, a_task);

	// Get task
	let l_task = match config::task_get(a_cfg, a_task)
	{
		Some(m_task) => m_task,
		None => return false,
	};

	// Task is not valid
	if !config::task_valid(a_task, &l_task)
	{
		return false;
	}

	// Do prepare
	if !do_prepare(a_cfg, a_task)
	{
		return false;
	}

	// Do rotation
	if !do_rotation(a_cfg, a_task)
	{
		return false;
	}

	// Do command
	if !do_command(a_cfg, a_task)
	{
		return false;
	}

	// Done
	return true;
}

/// Run tasks
///
/// Run all tasks from config.
fn tasks(a_cfg: &config::Config) -> bool
{
	// Hail
	println!("{}.* checking...", a_cfg.name);
	println!("");

	// Status bool
	let mut l_status = true;

	// Iterate over tasks
	for i_task in a_cfg.tasks.keys()
	{
		// Run task
		if !task(a_cfg, i_task.as_str())
		{
			l_status = false;
		}
	}

	// Done
	return l_status;
}
