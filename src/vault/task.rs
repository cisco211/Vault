// Use
use std::cmp::Ordering;
use std::env;
use std::process::Command;
use std::vec::Vec;
use chrono::{DateTime, Duration, Utc};
use crate::vault::config::{Config, ConfigTask};
use crate::vault::rotate::Rotate;
use crate::vault::state::State;
use crate::vault::time::Time;

/// Macros
pub const MACRO_NOW: &str = "{NOW}";
pub const MACRO_PATH: &str = "{PATH}";
pub const MACRO_STAMP: &str = "{STAMP}";

/// Task struct
#[derive(Clone)]
pub struct Task
{
	/// Cfg
	pub cfg: Config,

	/// Name
	pub name: String,

	/// Task
	pub task: ConfigTask,
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
				println!("Error: {}.{} failed to change directory into '{}'!\n{}", self.task.config, self.name, l_path_s, m_error.to_string());
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
			let l_str = Task::eval(i_cmd.as_str(), l_path_s, &l_now);

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

	/// Eval
	pub fn eval(a_cmd: &str, a_path: &str, a_stamp: &DateTime<Utc>) -> String
	{
		return a_cmd
			.replace(MACRO_NOW, Time::to_string(&Time::now()).as_str())
			.replace(MACRO_PATH, a_path)
			.replace(MACRO_STAMP, Time::to_string(a_stamp).as_str())
		;
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
		l_state.expires = Time::to_string(&(Time::now() + Duration::seconds(self.task.interval)));

		// Unlock
		l_state.locked = false;

		// Debug
		if self.cfg.debug
		{
			dbg!(&l_state);
		}

		// Save task
		if !State::save(&self.task.path, &l_state)
		{
			return false;
		}

		// Done
		println!("{}.{} done (next: {}).", self.cfg.name, self.name, l_state.expires);
		return true;
	}

	/// Prepare
	fn prepare(&mut self) -> bool
	{
		// Hail
		println!("{}.{} preparing...", self.cfg.name, self.name);

		// Get task
		self.task = match self.cfg.get_task(&self.name)
		{
			Some(m_task) => m_task,
			None => return false,
		};

		// Task not valid
		if !self.task.is_valid()
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

		// Debug
		if self.cfg.debug
		{
			dbg!(&l_state);
		}

		// Get expiration date
		let l_expires = match Time::from_string(l_state.expires.as_str())
		{
			Some(m_expires) => m_expires,
			None =>
			{
				println!("{}.{} skipped (invalid: {}).", self.cfg.name, self.name, l_state.expires);
				return false;
			},
		};

		// Not yet expired
		if Time::now() < l_expires
		{
			println!("{}.{} skipped (expires: {}).", self.cfg.name, self.name, l_state.expires);
			return false;
		}

		// Singleton
		if self.task.singleton
		{
			// Already locked
			if l_state.locked
			{
				println!("{}.{} skipped (locked).", self.cfg.name, self.name);
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
		// No rotate
		if !self.task.rotate.is_valid()
		{
			return true;
		}

		// Hail
		println!("{}.{} rotating...", self.cfg.name, self.name);

		// Create rotate
		let l_rotate = Rotate::new(self);

		// Run rotate
		return l_rotate.run();
	}

	/// Run
	pub fn run(a_cfg: &Config, a_task: &str) -> bool
	{
		// Create task
		let mut l_task = Task
		{
			cfg: a_cfg.clone(),
			name: a_task.to_string(),
			task: ConfigTask::default(),
		};

		// Empty task
		if l_task.name.is_empty()
		{
			println!("Error: Task name for configuration '{}' is empty!", a_cfg.name);
			return false;
		}

		// All tasks
		if l_task.name.eq("*")
		{
			return l_task.run_all();
		}

		// One specific task
		else
		{
			return l_task.run_one();
		}
	}

	/// Run all
	fn run_all(&mut self) -> bool
	{
		// Hail
		println!("{}.* checking...", self.cfg.name);

		// Status bool
		let mut l_status = true;

		// Copy values into a vector
		let mut l_tasks: Vec<ConfigTask> = self.cfg.tasks.values().cloned().collect();

		// Sort tasks by order
		l_tasks.sort_by(Task::sort_by_order);

		// Iterate over tasks
		for i_task in l_tasks
		{
			// Set name
			self.name = i_task.task;

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

	/// Sort by order
	fn sort_by_order(a_left: &ConfigTask, a_right: &ConfigTask) -> Ordering
	{
		if a_left.order < a_right.order
		{
			return Ordering::Less;
		}
		else if a_left.order == a_right.order
		{
			return Ordering::Equal;
		}
		else
		{
			return Ordering::Greater;
		}
	}

}
