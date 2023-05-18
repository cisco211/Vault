// Use
use serde::Deserialize;

// Config struct
#[derive(Debug, Deserialize)]
pub struct Config
{
	/// Config name
	pub name: String,
}

/// Get config
///
/// Get configuration from given path.
pub fn get(path: &std::path::Path) -> Option<Config>
{
	let p : std::path::PathBuf;
	if path.is_absolute()
	{
		p = path.to_path_buf();
	}
	else
	{
		match std::path::PathBuf::new().join(crate::vault::util::get_path_program()).join(path).canonicalize()
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
