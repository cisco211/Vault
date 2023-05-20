// Use
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use serde::Deserialize;

// Config struct
#[derive(Clone, Debug, Deserialize)]
pub struct Config
{
	/// Config name
	pub name: String,

	/// Tasks
	pub tasks: HashMap<String, ConfigTask>,
}

/// Config impl
impl Config
{
	/// Get task
	pub fn get_task(&self, a_task: &str) -> Option<ConfigTask>
	{
		if !self.tasks.contains_key(a_task)
		{
			println!("Error: {}.{} does not exist!", self.name, a_task);
			return None;
		}
		match self.tasks.get(a_task)
		{
			Some(m_task) =>
			{
				let mut l_task = m_task.clone();
				l_task.config = self.name.to_string();
				l_task.task = a_task.to_string();
				return Some(l_task);
			},
			None =>
			{
				println!("Error: {}.{} does not exist!", self.name, a_task);
				return None;
			},
		}
	}

	/// Load
	pub fn load(a_path: &Path) -> Option<Config>
	{
		let l_path: PathBuf;
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
			match env::current_dir()
			{
				Ok(m_path) =>
				{
					l_path = match PathBuf::new().join(m_path).join(a_path).canonicalize()
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
		let l_data = match fs::read_to_string(l_path)
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

// ConfigTask struct
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct ConfigTask
{
	/// Commands
	pub commands: Vec<String>,

	/// Config
	pub config: String,

	/// Enabled
	pub enabled: bool,

	/// Interval
	pub interval: i64,

	/// Path
	pub path: PathBuf,

	/// Singleton
	pub singleton: bool,

	/// Task
	pub task: String,
}

/// Default impl for Task
impl Default for ConfigTask
{
	/// Default
	fn default() -> ConfigTask
	{
		ConfigTask
		{
			commands: Vec::new(),
			config: String::new(),
			enabled: false,
			interval: 0,
			path: PathBuf::new(),
			singleton: true,
			task: String::new(),
		}
	}
}

/// Task impl
impl ConfigTask
{
	/// Valid
	pub fn valid(&self, a_cfg: &Config, a_task: &str) -> bool
	{
		// Task not enabled
		if !self.enabled
		{
			println!("{}.{} skipped (disabled).", a_cfg.name, a_task);
			return false;
		}

		// Negative interval
		if self.interval < 0
		{
			println!("{}.{} skipped (negative interval).", a_cfg.name, a_task);
			return false;
		}

		// No path
		match self.path.to_str()
		{
			Some(m_path) =>
			{
				if m_path.is_empty()
				{
					println!("{}.{} skipped (no path).", a_cfg.name, a_task);
					return false;
				}
			},
			None =>
			{
				println!("{}.{} skipped (no path).", a_cfg.name, a_task);
				return false;
			}
		}

		// Done
		return true;
	}
}
