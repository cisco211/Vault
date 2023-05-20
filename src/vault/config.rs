// Use
use crate::vault::time;

// Config struct
#[derive(Debug, serde::Deserialize)]
pub struct Config
{
	/// Config name
	pub name: String,

	/// Tasks
	pub tasks: std::collections::HashMap<String, Task>,
}

/// Config impl
impl Config
{
	/// Load
	pub fn load(a_path: &std::path::Path) -> Option<Config>
	{
		let l_path: std::path::PathBuf;
		if a_path.is_absolute()
		{
			l_path = a_path.to_path_buf();
			if !l_path.exists()
			{
				println!("Error: Configuration file '{}' does not exist!", a_path.display());
				return None;
			}
			if !l_path.is_file()
			{
				println!("Error: Configuration file '{}' is not a file!", a_path.display());
				return None;
			}
		}
		else
		{
			match std::env::current_dir()
			{
				Ok(m_path) =>
				{
					l_path = match std::path::PathBuf::new().join(m_path).join(a_path).canonicalize()
					{
						Ok(m_path) => m_path,
						Err(m_error) =>
						{
							println!("Error: Failed to canonicalize configuration file '{}'!\n{}", a_path.display(), m_error.to_string());
							return None;
						}
					};
				},
				Err(m_error) =>
				{
					println!("Error: Failed to get current directory for configuration file '{}'!\n{}", a_path.display(), m_error.to_string());
					return None;
				},
			}
		}
		let l_data = match std::fs::read_to_string(l_path)
		{
			Ok(m_data) => m_data,
			Err(m_error) =>
			{
				println!("Error: Failed to read configuration file '{}'!\n{}", a_path.display(), m_error.to_string());
				return None;
			}
		};
		match toml::from_str(l_data.as_str())
		{
			Ok(m_config) => return Some(m_config),
			Err(m_error) =>
			{
				println!("Error: Failed to parse configuration file '{}'!\n{}", a_path.display(), m_error.to_string());
				return None;
			}
		}
	}

}

// Task struct
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Task
{
	/// Commands
	pub commands: std::vec::Vec<String>,

	/// Config
	pub config: String,

	/// Enabled
	pub enabled: bool,

	/// Interval
	pub interval: i64,

	/// Path
	pub path: std::path::PathBuf,

	/// Singleton
	pub singleton: bool,

	/// Task
	pub task: String,
}

/// Default impl for Task
impl Default for Task
{
	/// Default
	fn default() -> Task
	{
		Task
		{
			commands: std::vec::Vec::new(),
			config: String::new(),
			enabled: false,
			interval: 0,
			path: std::path::PathBuf::new(),
			singleton: true,
			task: String::new(),
		}
	}
}

/// Task impl
impl Task
{
	/// Eval
	pub fn eval(a_cmd: &String, a_path: &str, a_stamp: chrono::DateTime<chrono::Utc>) -> String
	{
		return a_cmd
			.replace("{NOW}", time::Time::to_string(time::Time::now()).as_str())
			.replace("{PATH}", a_path)
			.replace("{STAMP}", time::Time::to_string(a_stamp).as_str())
		;
	}

	/// Get
	pub fn get(a_cfg: &Config, a_task: &str) -> Option<Task>
	{
		if !a_cfg.tasks.contains_key(a_task)
		{
			println!("Error: {}.{} does not exist!", a_cfg.name, a_task);
			return None;
		}
		match a_cfg.tasks.get(a_task)
		{
			Some(m_task) =>
			{
				let mut l_task = m_task.clone();
				l_task.config = a_cfg.name.to_string();
				l_task.task = a_task.to_string();
				return Some(l_task);
			},
			None =>
			{
				println!("Error: {}.{} does not exist!", a_cfg.name, a_task);
				return None;
			},
		}
	}

	/// Valid
	pub fn valid(&self, a_cfg: &Config, a_task: &str) -> bool
	{
		// Task not enabled
		if !self.enabled
		{
			println!("{}.{} skipped (disabled).\n", a_cfg.name, a_task);
			return false;
		}

		// Negative interval
		if self.interval < 0
		{
			println!("{}.{} skipped (negative interval).\n", a_cfg.name, a_task);
			return false;
		}

		// No path
		match self.path.to_str()
		{
			Some(m_path) =>
			{
				if m_path.is_empty()
				{
					println!("{}.{} skipped (no path).\n", a_cfg.name, a_task);
					return false;
				}
			},
			None =>
			{
				println!("{}.{} skipped (no path).\n", a_cfg.name, a_task);
				return false;
			}
		}

		// Done
		return true;
	}
}

/// Tests mod
mod tests
{
	/// Smoke
	#[test]
	fn smoke()
	{}
}
