// Mod
mod args;
mod config;
mod task;
mod util;

/// Help
fn help(a_long: bool)
{
	println!("");
	if a_long
	{
		match args::Args::cmd().print_long_help()
		{
			Ok(_) => (),
			Err(_) => println!("Error: Failed to show long help text!"),
		}
	}
	else
	{
		match args::Args::cmd().print_help()
		{
			Ok(_) => (),
			Err(_) => println!("Error: Failed to show short help text!"),
		}
	}
}

/// Run
pub fn run() -> bool
{
	// Get arguments
	let l_args = args::Args::read();

	// Config given
	if let Some(l_config) = l_args.config.as_deref()
	{
		// Load configuration
		let l_cfg = match config::Config::load(l_config)
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
				println!("Vault at {}\n", util::Time::to_string(util::Time::now()));
				return task_all(&l_cfg);
			}

			// One specific task
			else
			{
				println!("Vault at {}\n", util::Time::to_string(util::Time::now()));
				return task_one(&l_cfg, l_task);
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

/// Task one
fn task_one(a_cfg: &config::Config, a_task: &str) -> bool
{
	// Hail
	println!("{}.{} checking...", a_cfg.name, a_task);

	// Do prepare
	if !task_prepare(a_cfg, a_task)
	{
		return false;
	}

	// Do rotation
	if !task_rotation(a_cfg, a_task)
	{
		return false;
	}

	// Do command
	if !task_command(a_cfg, a_task)
	{
		return false;
	}

	// Done
	return true;
}

/// Task all
fn task_all(a_cfg: &config::Config) -> bool
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
		if !task_one(a_cfg, i_task.as_str())
		{
			l_status = false;
		}
	}

	// Done
	return l_status;
}

/// Task command
fn task_command(a_cfg: &config::Config, a_task: &str) -> bool
{
	// Hail
	println!("{}.{} executing...", a_cfg.name, a_task);

	// Get task
	let l_task_c = match config::Task::get(a_cfg, a_task)
	{
		Some(m_task) => m_task,
		None => return false,
	};

	// Create command
	let mut l_cmd = std::process::Command::new(l_task_c.cmd.clone());

	// Set working directory
	l_cmd.current_dir(l_task_c.path.clone());

	// Now
	let l_now = util::Time::now();

	// Add arguments
	for i_value in l_task_c.args
	{
		l_cmd.arg(i_value.replace("{NOW}", util::Time::to_string(l_now).as_str()));
	}

	// Execute command
	let l_status = match l_cmd.status()
	{
		Ok(m_status) => m_status,
		Err(m_error) =>
		{
			println!("Error: {}.{} failed to execute command '{}'!\n{}", l_task_c.config, a_task, l_task_c.cmd, m_error.to_string());
			return false;
		}
	};

	// Execution failed
	if !l_status.success()
	{
		println!("Error: {}.{} failed to execute command '{}'!", l_task_c.config, a_task, l_task_c.cmd);
		return false;
	}

	// Get task file path
	let l_path = std::path::PathBuf::new().join(l_task_c.path);

	// Load task
	let mut l_task_t = match task::Task::load(&l_path)
	{
		Some(m_task) => m_task,
		None => return false,
	};

	// Update expiration date
	l_task_t.expires = util::Time::to_string(l_now + chrono::Duration::seconds(l_task_c.interval));

	// Save task
	if !task::Task::save(&l_path, &l_task_t)
	{
		return false;
	}

	// Done
	println!("{}.{} done (next: {}).\n", a_cfg.name, a_task, l_task_t.expires);
	return true;
}

/// Task prepare
fn task_prepare(a_cfg: &config::Config, a_task: &str) -> bool
{
	// Hail
	println!("{}.{} preparing...", a_cfg.name, a_task);

	// Get config task
	let l_task_c = match config::Task::get(a_cfg, a_task)
	{
		Some(m_task) => m_task,
		None => return false,
	};

	// Task not valid
	if !l_task_c.valid(a_cfg, a_task)
	{
		return false;
	}

	// Create task if not exist
	if !task::Task::create(&l_task_c.path)
	{
		return false;
	}

	// Load task
	let l_task_t = match task::Task::load(&l_task_c.path)
	{
		Some(m_task) => m_task,
		None => return false,
	};

	// Get expiration date
	let l_expires = match util::Time::from_string(l_task_t.expires.as_str())
	{
		Some(m_expires) => m_expires,
		None => return false,
	};

	// Not yet expired
	if util::Time::now() < l_expires
	{
		println!("{}.{} skipped (expires: {}).\n", a_cfg.name, a_task, l_task_t.expires);
		return false;
	}

	// Done
	return true;
}

/// Task rotation
fn task_rotation(a_cfg: &config::Config, a_task: &str) -> bool
{
	// Hail
	println!("{}.{} rotating...", a_cfg.name, a_task);

	// TODO: Implement do_rotation.
	true
}
