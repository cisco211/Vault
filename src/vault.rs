// Mod
mod args;
mod config;
mod state;
mod task;
mod time;

// Use
use crate::vault::args::Args;
use crate::vault::config::Config;
use crate::vault::task::Task;
use crate::vault::time::Time;

/// Help
fn help(a_long: bool)
{
	println!("");
	if a_long
	{
		match Args::cmd().print_long_help()
		{
			Ok(_) => (),
			Err(_) => println!("Error: Failed to show long help text!"),
		}
	}
	else
	{
		match Args::cmd().print_help()
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
	let l_args = Args::read();

	// Config given
	if let Some(l_config) = l_args.config
	{
		// Load configuration
		let mut l_cfg = match Config::load(&l_config)
		{
			Some(m_cfg) => m_cfg,
			None => return false,
		};

		// Debug
		l_cfg.debug = l_args.debug;
		if l_args.debug
		{
			dbg!(&l_cfg);
		}

		// No name
		if l_cfg.name.is_empty()
		{
			println!("Error: Configuration file '{}' has no name!", l_config.display());
			return false;
		}

		// Task given
		if let Some(l_task) = l_args.task
		{
			println!("vault at {}", Time::to_string(&Time::now()));
			let l_status = Task::run(&l_cfg, l_task.as_str());
			println!("");
			return l_status;
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
