// Use
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
pub fn load(a_path: &std::path::Path) -> Option<Config>
{
	let l_path: std::path::PathBuf;
	if a_path.is_absolute()
	{
		l_path = a_path.to_path_buf();
	}
	else
	{
		l_path = match std::path::PathBuf::new().join(util::path_program()).join(a_path).canonicalize()
		{
			Ok(m_path) => m_path,
			Err(m_error) =>
			{
				println!("Error: Failed to canonicalize configuration file '{}'!\n{}", a_path.display(), m_error.to_string());
				return None;
			}
		};
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

/// Task get
pub fn task_get(a_cfg: &Config, a_task: &str) -> Option<toml::map::Map<String, toml::Value>>
{
	if !a_cfg.tasks.contains_key(a_task)
	{
		println!("Error: Configuration '{}' task '{}' does not exist!", a_cfg.name, a_task);
		return None;
	}
	match a_cfg.tasks[a_task].as_table()
	{
		Some(m_task) => Some(m_task.clone()),
		None =>
		{
			println!("Error: Configuration '{}' task '{}' is not readable!", a_cfg.name, a_task);
			return None;
		}
	}
}

/// Task valid
pub fn task_valid(a_key: &str, a_value: &toml::map::Map<String, toml::Value>) -> bool
{
	// Args not found
	if !a_value.contains_key("args")
	{
		println!("Error: Task '{}' does not have an args array!", a_key);
		return false;
	}

	// Args is not array
	if !a_value["args"].is_array()
	{
		println!("Error: Task '{}' args is not an array!", a_key);
		return false;
	}

	// Cmd not found
	if !a_value.contains_key("cmd")
	{
		println!("Error: Task '{}' does not have a cmd string!", a_key);
		return false;
	}

	// Cmd is not string
	if !a_value["cmd"].is_str()
	{
		println!("Error: Task '{}' cmd is not a string!", a_key);
		return false;
	}

	// Interval not found
	if !a_value.contains_key("interval")
	{
		println!("Error: Task '{}' does not have an interval integer!", a_key);
		return false;
	}

	// Interval not integer
	if !a_value["interval"].is_integer()
	{
		println!("Error: Task '{}' interval is not an integer!", a_key);
		return false;
	}

	// Path not found
	if !a_value.contains_key("path")
	{
		println!("Error: Task '{}' does not have a path string!", a_key);
		return false;
	}

	// Path is not string
	if !a_value["path"].is_str()
	{
		println!("Error: Task '{}' path is not a string!", a_key);
		return false;
	}

	// Done
	return true;
}
