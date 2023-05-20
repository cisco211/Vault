// Mod
mod args;
mod config;
mod state;
mod time;

// Use
use std::env;
use std::process::Command;
use std::vec::Vec;
use chrono::Duration;
use crate::vault::args::Args;
use crate::vault::config::Config;
use crate::vault::config::Task as ConfigTask;
use crate::vault::state::State;
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
	if let Some(l_config) = l_args.config.as_deref()
	{
		// Load configuration
		let l_cfg = match Config::load(l_config)
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
			return Task::run(l_cfg, l_task);
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

/// Task struct
struct Task
{
	/// Cfg
	cfg: config::Config,

	/// Name
	name: String,

	/// Task
	task: config::Task,
}

/// Task impl
impl Task
{
	/// Command
	fn command(&self) -> bool
	{
		// Hail
		println!("{}.{} executing...", self.cfg.name, self.name);

		// Get task file path
		let l_path = self.task.path.clone();
		let l_path_s = match l_path.to_str()
		{
			Some(m_str) => m_str,
			None => return false,
		};

		// Failed to change dir
		match env::set_current_dir(l_path.clone())
		{
			Ok(_) => {},
			Err(m_error) =>
			{
				println!("Error: {}.{} failed to cd into '{}'!\n{}", self.task.config, self.name, l_path_s, m_error.to_string());
				return false;
			}
		}

		// Now
		let l_now = Time::now();

		// Iterate over commands
		for i_cmd in self.task.commands.iter()
		{
			// No command
			if i_cmd.is_empty()
			{
				continue;
			}

			// Eval command
			let l_str = ConfigTask::eval(&i_cmd, l_path_s, l_now);

			// Split command
			let l_split = l_str.split(" ").collect::<Vec<&str>>();

			// No split
			if l_split.is_empty()
			{
				continue;
			}

			// Create command
			let mut l_cmd = Command::new(l_split[0]);

			// Set working directory
			l_cmd.current_dir(l_path.clone());

			// Add arguments
			for i_index in 1..l_split.len()
			{
				let l_str = l_split[i_index]
					;
				l_cmd.arg(l_str);
			}

			// Execute command
			let l_status = match l_cmd.status()
			{
				Ok(m_status) => m_status,
				Err(m_error) =>
				{
					println!("Error: {}.{} failed to execute command '{}'!\n{}", self.task.config, self.name, l_split[0], m_error.to_string());
					return false;
				}
			};

			// Execution failed
			if !l_status.success()
			{
				println!("Error: {}.{} failed to execute command '{}'!", self.task.config, self.name, l_split[0]);
				return false;
			}
		}

		// Done
		println!("{}.{} executed.", self.cfg.name, self.name);
		return true;
	}

	/// Finalize
	fn finalize(&self) -> bool
	{
		// Load state
		let mut l_state = match State::load(&self.task.path)
		{
			Some(m_state) => m_state,
			None => return false,
		};

		// Update expiration date
		l_state.expires = Time::to_string(Time::now() + Duration::seconds(self.task.interval));

		// Unlock
		l_state.locked = false;

		// Save task
		if !State::save(&self.task.path, &l_state)
		{
			return false;
		}

		// Done
		println!("{}.{} done (next: {}).\n", self.cfg.name, self.name, l_state.expires);
		return true;
	}

	/// Prepare
	fn prepare(&mut self) -> bool
	{
		// Hail
		println!("{}.{} preparing...", self.cfg.name, self.name);

		// Get task
		self.task = match ConfigTask::get(&self.cfg, &self.name)
		{
			Some(m_task) => m_task,
			None => return false,
		};

		// Task not valid
		if !self.task.valid(&self.cfg, &self.name)
		{
			return false;
		}

		// Create state if not exist
		if !State::create(&self.task.path)
		{
			return false;
		}

		// Load state
		let l_state = match State::load(&self.task.path)
		{
			Some(m_state) => m_state,
			None => return false,
		};

		// Get expiration date
		let l_expires = match Time::from_string(l_state.expires.as_str())
		{
			Some(m_expires) => m_expires,
			None =>
			{
				println!("{}.{} skipped (invalid: {}).\n", self.cfg.name, self.name, l_state.expires);
				return false;
			},
		};

		// Not yet expired
		if Time::now() < l_expires
		{
			println!("{}.{} skipped (expires: {}).\n", self.cfg.name, self.name, l_state.expires);
			return false;
		}

		// Singleton
		if self.task.singleton
		{
			// Already locked
			if l_state.locked
			{
				println!("{}.{} skipped (locked).\n", self.cfg.name, self.name);
				return false;
			}

			// Lock
			else
			{
				let mut l_state = l_state.clone();
				l_state.locked = true;
				if !State::save(&self.task.path, &l_state)
				{
					return false;
				}
			}
		}

		// Done
		return true;
	}

	/// Rotate
	fn rotate(&self) -> bool
	{
		// Hail
		//println!("{}.{} rotating...", self.cfg.name, self.name);

		// TODO: Implement task_rotate.
		true
	}

	/// Run
	fn run(a_cfg: config::Config, a_task: &str) -> bool
	{
		// Create task
		let mut l_task = Task
		{
			cfg: a_cfg,
			name: a_task.to_string(),
			task: ConfigTask::default(),
		};

		// Empty task
		if l_task.name.is_empty()
		{
			println!("Error: Task is empty!");
			return false;
		}

		// All tasks
		if l_task.name.eq("*")
		{
			println!("Vault at {}\n", Time::to_string(Time::now()));
			return l_task.run_all();
		}

		// One specific task
		else
		{
			println!("Vault at {}\n", Time::to_string(Time::now()));
			return l_task.run_one();
		}
	}

	/// Run all
	fn run_all(&mut self) -> bool
	{
		// Hail
		println!("{}.* checking...", self.cfg.name);
		println!("");

		// Status bool
		let mut l_status = true;

		// Clone config
		let l_cfg = self.cfg.clone();

		// Iterate over tasks
		for i_task in l_cfg.tasks.keys()
		{
			// Set name
			self.name = i_task.clone();

			// Run task
			if !self.run_one()
			{
				l_status = false;
			}
		}

		// Done
		return l_status;
	}

	/// Run one
	fn run_one(&mut self) -> bool
	{
		// Prepare
		if !self.prepare()
		{
			return false;
		}

		// Command
		if !self.command()
		{
			return self.finalize();
		}

		// Rotate
		if !self.rotate()
		{
			return self.finalize();
		}

		// Finalize
		if !self.finalize()
		{
			return false;
		}

		// Done
		return true;
	}
}
