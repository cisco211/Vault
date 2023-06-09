// Use
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::vec::Vec;
use serde::Deserialize;

// Config struct
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config
{
	/// Debug
	pub debug: bool,

	/// Name
	pub name: String,

	/// Tasks
	pub tasks: HashMap<String, ConfigTask>,
}

/// Default impl for Config
impl Default for Config
{
	/// Default
	fn default() -> Config
	{
		Config
		{
			debug: false,
			name: String::new(),
			tasks: HashMap::new(),
		}
	}
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
			Some(m_task) => return Some(m_task.clone()),
			None =>
			{
				println!("Error: {}.{} does not exist!", self.name, a_task);
				return None;
			},
		}
	}

	/// Load
	pub fn load(a_path: &PathBuf) -> Option<Config>
	{
		// Path
		let l_path: PathBuf;

		// Absolute path
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

		// Relative path
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

		// Get data from file
		let l_data = match fs::read_to_string(l_path)
		{
			Ok(m_data) => m_data,
			Err(m_error) =>
			{
				println!("Error: Failed to read configuration file '{}'!\n{}", a_path.display(), m_error.to_string());
				return None;
			}
		};

		// Parse into config
		let mut l_config: Config = match toml::from_str(l_data.as_str())
		{
			Ok(m_config) => m_config,
			Err(m_error) =>
			{
				println!("Error: Failed to parse configuration file '{}'!\n{}", a_path.display(), m_error.to_string());
				return None;
			}
		};

		// Iterate over task and assign their config and task strings
		for (i_k, i_v) in l_config.tasks.iter_mut()
		{
			i_v.config = l_config.name.clone();
			i_v.task = i_k.clone();
		}

		// Done
		return Some(l_config);
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

	/// Order
	pub order: u64,

	/// Path
	pub path: PathBuf,

	/// Rotate
	pub rotate: ConfigTaskRotate,

	/// Rotate strategy
	pub rotate_strategy: String,

	/// Singleton
	pub singleton: bool,

	/// Task
	pub task: String,
}

/// Default impl for ConfigTask
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
			order: 0,
			path: PathBuf::new(),
			rotate: ConfigTaskRotate::default(),
			rotate_strategy: String::from("move"),
			singleton: true,
			task: String::new(),
		}
	}
}

/// Task impl
impl ConfigTask
{
	/// Valid
	pub fn is_valid(&self) -> bool
	{
		// Task not enabled
		if !self.enabled
		{
			println!("{}.{} skipped (disabled).", self.config, self.task);
			return false;
		}

		// Negative interval
		if self.interval < 0
		{
			println!("{}.{} skipped (negative interval).", self.config, self.task);
			return false;
		}

		// No path
		match self.path.to_str()
		{
			Some(m_path) =>
			{
				if m_path.is_empty()
				{
					println!("{}.{} skipped (no path).", self.config, self.task);
					return false;
				}
			},
			None =>
			{
				println!("{}.{} skipped (no path).", self.config, self.task);
				return false;
			}
		}

		// Done
		return true;
	}
}

/// ConfigTaskRotate struct
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct ConfigTaskRotate
{
	/// Daily
	pub daily: bool,

	/// Hourly
	pub hourly: bool,

	/// Monthly
	pub monthly: bool,

	/// Yearly
	pub yearly: bool,
}

/// Default impl for ConfigTaskRotate
impl Default for ConfigTaskRotate
{
	/// Default
	fn default() -> ConfigTaskRotate
	{
		ConfigTaskRotate
		{
			daily: false,
			hourly: false,
			monthly: false,
			yearly: false,
		}
	}
}

/// ConfigTaskRotate impl
impl ConfigTaskRotate
{
	/// Is valid
	pub fn is_valid(&self) -> bool
	{
		self.daily || self.hourly || self.monthly || self.yearly
	}
}
