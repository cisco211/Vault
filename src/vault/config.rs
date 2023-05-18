// Use
use crate::vault::config;
use crate::vault::util;

// Config struct
#[derive(Debug, serde::Deserialize)]
pub struct Config
{
	/// Config name
	pub name: String,

	/// Tasks
	pub tasks: toml::Table,
}

/// Load
pub fn load(path: &std::path::Path) -> Option<Config>
{
	let p : std::path::PathBuf;
	if path.is_absolute()
	{
		p = path.to_path_buf();
	}
	else
	{
		match std::path::PathBuf::new().join(util::get_path_program()).join(path).canonicalize()
		{
			Ok(o) => p = o,
			Err(e) =>
			{
				println!("Error: Failed to canonicalize configuration file '{}'!\n{}", path.display(), e.to_string());
				return None;
			}
		}
	}
	match std::fs::read_to_string(p)
	{
		Ok(s) =>
		{
			match toml::from_str(s.as_str())
			{
				Ok(c) => return Some(c),
				Err(e) =>
				{
					println!("Error: Failed to parse configuration file '{}'!\n{}", path.display(), e.to_string());
					return None;
				}
			}
		}
		Err(e) =>
		{
			println!("Error: Failed to read configuration file '{}'!\n{}", path.display(), e.to_string());
			return None;
		}
	}
}

/// Task get
pub fn task_get(cfg: &config::Config, task: &str) -> Option<toml::map::Map<String, toml::Value>>
{
	if !cfg.tasks.contains_key(task)
	{
		println!("Error: Configuration '{}' task '{}' does not exist!", cfg.name, task);
		return None;
	}
	match cfg.tasks[task].as_table()
	{
		Some(t_) => Some(t_.clone()),
		None =>
		{
			println!("Error: Configuration '{}' task '{}' is not readable!", cfg.name, task);
			return None;
		}
	}
}

pub fn task_valid(key: &str, value: &toml::map::Map<String, toml::Value>) -> bool
{
		// Key cmd not found
		if !value.contains_key("cmd")
		{
			println!("Error: Task '{}' does not have a cmd key!", key);
			return false;
		}

		// Value cmd is not string
		if !value["cmd"].is_str()
		{
			println!("Error: Task '{}' cmd value is not a string!", key);
			return false;
		}

		// Key args not found
		if !value.contains_key("args")
		{
			println!("Error: Task '{}' does not have an args key!", key);
			return false;
		}

		// Value args is not array
		if !value["args"].is_array()
		{
			println!("Error: Tsk '{}' args value is not an array!", key);
			return false;
		}

		// Done
		return true;
}
