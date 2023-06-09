// Use
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::vault::time::Time;

/// State struct
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct State
{
	/// Expires
	pub expires: String,

	/// Locked
	pub locked: bool,
}

/// Default impl for State
impl Default for State
{
	/// Default
	fn default() -> State
	{
		State
		{
			expires: Time::to_string(&Time::now()),
			locked: false,
		}
	}
}

/// State impl
impl State
{
	/// Create
	pub fn create(a_path: &PathBuf) -> bool
	{
		// Path does not exist
		if !a_path.exists()
		{
			// Create directory recursively
			match fs::create_dir_all(a_path.to_path_buf())
			{
				Ok(_) => {},
				Err(m_error) =>
				{
					println!("Error: Failed to create path '{}'!\n{}", a_path.display(), m_error.to_string());
					return false;
				}
			}
		}

		// Get state file path
		let l_path = State::path(a_path);

		// State file already exists
		if l_path.exists()
		{
			return true;
		}

		// Create state file
		match fs::write(l_path, "")
		{
			Ok(_) => {},
			Err(m_error) =>
			{
				println!("Error: Failed to create state file '{}'!\n{}", a_path.display(), m_error.to_string());
				return false;
			}
		}

		// Done
		return State::save(&a_path, &State::default());
	}

	/// Load
	pub fn load(a_path: &PathBuf) -> Option<State>
	{
		// Get path
		let l_path = match State::path(a_path).canonicalize()
		{
			Ok(m_path) => m_path,
			Err(m_error) =>
			{
				println!("Error: Failed to canonicalize state file '{}'!\n{}", a_path.display(), m_error.to_string());
				return None;
			}
		};

		// Read file
		let l_data = match fs::read_to_string(l_path)
		{
			Ok(m_data) => m_data,
			Err(m_error) =>
			{
				println!("Error: Failed to read state file '{}'!\n{}", a_path.display(), m_error.to_string());
				return None;
			}
		};

		// Deserialize state
		match toml::from_str(l_data.as_str())
		{
			Ok(m_state) => return Some(m_state),
			Err(m_error) =>
			{
				println!("Error: Failed to parse state file '{}'!\n{}", a_path.display(), m_error.to_string());
				return None;
			}
		}
	}

	/// Path
	pub fn path(a_path: &PathBuf) -> PathBuf
	{
		return PathBuf::new().join(a_path).join("state.toml");
	}

	/// Save
	pub fn save(a_path: &PathBuf, a_state: &State) -> bool
	{
		// Get path
		let l_path = match State::path(a_path).canonicalize()
		{
			Ok(m_path) => m_path,
			Err(m_error) =>
			{
				println!("Error: Failed to canonicalize state file '{}'!\n{}", a_path.display(), m_error.to_string());
				return false;
			}
		};

		// Serialize state
		let l_data = match toml::to_string(&a_state)
		{
			Ok(m_data) => m_data,
			Err(m_error) =>
			{
				println!("Error: Failed to construct state file '{}'!\n{}", a_path.display(), m_error.to_string());
				return false;
			}
		};

		// Write file
		match fs::write(l_path, l_data)
		{
			Ok(_) => return true,
			Err(m_error) =>
			{
				println!("Error: Failed to write state file '{}'!\n{}", a_path.display(), m_error.to_string());
				return false;
			}
		}
	}
}
