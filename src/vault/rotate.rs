// Use
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::vec::Vec;
use crate::vault::task::Task;

// Directory move
pub const DIRECTORY_MOVE: &str = "moved";

// Regular expressions
pub const REGEXP_DAILY: &str = "^([0-9]{4}-[0-9]{2}-[0-9]{2})";
pub const REGEXP_HOURLY: &str = "^([0-9]{4}-[0-9]{2}-[0-9]{2}_[0-9]{2})";
pub const REGEXP_MONTHLY: &str = "^([0-9]{4}-[0-9]{2})";
pub const REGEXP_YEARLY: &str = "^([0-9]{4})";

// Rotate struct
pub struct Rotate
{
	// Operate function
	operate: fn(&Rotate, &str) -> bool,

	/// Task
	task: Task,
}

// Rotate impl
impl Rotate
{
	/// File delete
	fn file_delete(&self, a_file: &str) -> bool
	{
		match fs::remove_file(PathBuf::new().join(self.task.task.path.clone()).join(a_file))
		{
			Ok(_) => return true,
			Err(m_error) =>
			{
				println!("Error: {}.{} failed to delete file '{}'!\n{}", self.task.cfg.name, self.task.name, a_file, m_error.to_string());
				return false;
			},
		}
	}

	/// File move
	fn file_move(&self, a_file: &str) -> bool
	{
		let l_source = PathBuf::new().join(self.task.task.path.clone()).join(a_file);
		let l_target = PathBuf::new().join(self.task.task.path.clone()).join(DIRECTORY_MOVE).join(a_file);
		match fs::rename(l_source, l_target)
		{
			Ok(_) => return true,
			Err(m_error) =>
			{
				println!("Error: {}.{} failed to move file '{}' into '{}/{}'!\n{}", self.task.cfg.name, self.task.name, a_file, DIRECTORY_MOVE, a_file, m_error.to_string());
				return false;
			}
		}
	}

	/// File unknown
	fn file_unknown(&self, _a_file: &str) -> bool
	{
		// Always return false
		return false;
	}

	/// List files
	fn list_files(&self) -> Vec<String>
	{
		let mut l_list = Vec::<String>::new();
		let l_items = match fs::read_dir(self.task.task.path.clone())
		{
			Ok(m_items) => m_items,
			Err(m_error) =>
			{
				println!("Error: {}.{} failed to read directory '{}'!\n{}", self.task.cfg.name, self.task.name, self.task.task.path.display(), m_error.to_string());
				return l_list;
			},
		};
		for i_item in l_items
		{
			match i_item
			{
				Ok(m_item) =>
				{
					let l_file;
					match m_item.file_name().to_str()
					{
						Some(m_file) => l_file = m_file.to_string(),
						None => continue,
					};

					l_list.push(l_file);
				},
				Err(_m_error) => {},
			}
		}
		return l_list;
	}

	/// List filters
	fn list_filters(&self, a_list: &Vec<String>, a_filter: &str) -> Vec<String>
	{
		let mut l_list = Vec::<String>::new();
		for i_item in a_list
		{
			if i_item.starts_with(a_filter)
			{
				l_list.push(i_item.clone());
			}
		}
		return l_list;
	}

	/// List tree
	fn list_tree(&self, a_expr: &str) -> Option<BTreeMap::<String, Vec<String>>>
	{
		// Create tree
		let mut l_tree = BTreeMap::<String, Vec<String>>::new();

		// Get list of files
		let l_files = self.list_files();

		// List is empty
		if l_files.is_empty()
		{
			return Some(l_tree);
		}

		// Create regular expression
		let l_regex = match Regex::new(a_expr)
		{
			Ok(m_regex) => m_regex,
			Err(m_error) =>
			{
				println!("Error: {}.{} invalid regular expression '{}'!\n{}", self.task.cfg.name, self.task.name, a_expr, m_error.to_string());
				return None;
			}
		};

		// Get filters
		let l_filters = self.list_uniques(&l_files, &l_regex);

		// Iterate over filters
		for i_filter in l_filters
		{
			// Insert filtered file list into tree
			l_tree.insert(i_filter.clone(), self.list_filters(&l_files, &i_filter));
		}

		// Done
		return Some(l_tree);
	}

	/// List uniques
	fn list_uniques(&self, a_list: &Vec<String>, a_regex: &Regex) -> Vec<String>
	{
		let mut l_list = Vec::<String>::new();
		for i_item in a_list
		{
			match a_regex.captures(i_item)
			{
				Some(m_match) => l_list.push(m_match[1].to_string()),
				None => {},
			};
		}
		l_list.dedup();
		return l_list;
	}

	// New
	pub fn new(a_task: &Task) -> Rotate
	{
		return Rotate
		{
			operate: match a_task.task.rotate_strategy.as_str()
			{
				"delete" => Rotate::file_delete,
				"move" => Rotate::file_move,
				_ => Rotate::file_unknown,
			},
			task: a_task.clone()
		};
	}

	/// Rotate
	fn rotate(&self, a_expr: &str) -> bool
	{
		// Get tree
		let mut l_tree = match self.list_tree(a_expr)
		{
			Some(m_map) => m_map,
			None => return false,
		};

		// Tree is empty
		if l_tree.is_empty()
		{
			return true;
		}

		// Remove last entry
		l_tree.pop_last();

		// Debug
		{
			dbg!(a_expr);
			dbg!(l_tree.clone());
		}

		// Iterate over tree
		for (_i_k, i_v) in &l_tree
		{
			// No files
			if i_v.is_empty()
			{
				continue;
			}

			// Already rotated
			if i_v.len() == 1
			{
				continue;
			}

			// Clone file list
			let mut l_files = i_v.clone();

			// Remove last entry
			l_files.pop();

			// Iterate over file list
			for i_file in &l_files
			{
				// Operate on that file
				if !(self.operate)(&self, i_file)
				{
					return false;
				}
			}
		}

		// Done
		return true;
	}

	/// Run
	pub fn run(&self) -> bool
	{
		// Choose strategy
		match self.task.task.rotate_strategy.as_str()
		{
			// Delete
			"delete" => {},

			// Move
			"move" =>
			{
				let l_path = PathBuf::new().join(self.task.task.path.clone()).join(DIRECTORY_MOVE);

				// Moved directory does not exist
				if !l_path.exists()
				{
					// Create moved directory
					match fs::create_dir(l_path.clone())
					{
						Ok(_) => {},
						Err(m_error) =>
						{
							println!("Error: {}.{} failed to create directory '{}'!\n{}", self.task.cfg.name, self.task.name, l_path.display(), m_error.to_string());
							return false;
						}
					}
				}
			},

			// Unknown
			_ =>
			{
				println!("Error: {}.{} unknown rotate strategy '{}'!", self.task.cfg.name, self.task.name, self.task.task.rotate_strategy);
				return false;
			},
		}

		// Run hourly
		if !self.run_hourly()
		{
			return false;
		}

		// Run daily
		if !self.run_daily()
		{
			return false;
		}

		// Run monthly
		if !self.run_monthly()
		{
			return false;
		}

		// Run yearly
		if !self.run_yearly()
		{
			return false;
		}

		// Done
		return true;
	}

	/// Run daily
	fn run_daily(&self) -> bool
	{
		// No daily
		if !self.task.task.rotate.daily
		{
			return true;
		}

		// Perform rotation
		return self.rotate(REGEXP_DAILY);
	}

	/// Run hourly
	fn run_hourly(&self) -> bool
	{
		// No daily
		if !self.task.task.rotate.hourly
		{
			return true;
		}

		// Perform rotation
		return self.rotate(REGEXP_HOURLY);
	}

	/// Run monthly
	fn run_monthly(&self) -> bool
	{
		// No daily
		if !self.task.task.rotate.monthly
		{
			return true;
		}

		// Perform rotation
		return self.rotate(REGEXP_MONTHLY);
	}

	/// Run yearly
	fn run_yearly(&self) -> bool
	{
		// No daily
		if !self.task.task.rotate.yearly
		{
			return true;
		}

		// Perform rotation
		return self.rotate(REGEXP_YEARLY);
	}
}
