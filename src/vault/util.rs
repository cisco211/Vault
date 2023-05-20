/// Path struct
pub struct Path
{}

// Path impl
impl Path
{
	/// List
	#[allow(dead_code)]
	pub fn list(a_path: &std::path::PathBuf) -> std::vec::Vec<std::path::PathBuf>
	{
		let mut l_data = std::vec::Vec::<std::path::PathBuf>::new();
		match std::fs::read_dir(a_path)
		{
			Ok(m_dir) =>
			{
				for m_item in m_dir
				{
					match m_item
					{
						Ok(m_entry) => l_data.push(m_entry.path()),
						Err(_m_error) => continue
					}
				}
			},
			Err(_m_error) => {},
		}
		return l_data;
	}

	/// Program
	pub fn program() -> std::path::PathBuf
	{
		match std::env::current_exe()
		{
			Ok(m_path) =>
			{
				match m_path.parent()
				{
					Some(m_parent) => return m_parent.to_path_buf(),
					None => return std::path::PathBuf::from("."),
				}
			},
			Err(_m_error) => return std::path::PathBuf::from("."),
		}
	}
}

/// Time struct
pub struct Time
{}

/// Time impl
impl Time
{
	/// from string
	pub fn from_string(a_str: &str) -> Option<chrono::DateTime<chrono::Utc>>
	{
		use chrono::TimeZone;
		match chrono::Utc.datetime_from_str(a_str, "%Y-%m-%d.%H-%M-%S%.f")
		{
			Ok(m_dt) => return Some(m_dt),
			Err(_m_error) => return None,
		}
	}

	/// Now
	pub fn now() -> chrono::DateTime<chrono::Utc>
	{
		return chrono::offset::Utc::now();
	}

	/// To string
	pub fn to_string(a_dt: chrono::DateTime<chrono::Utc>) -> String
	{
		return a_dt.to_string().replace(" ", ".").replace(":", "-").replace(".UTC", "");
	}

}

/// Tests module
mod tests
{
	/// Smoke path
	#[test]
	fn smoke_path()
	{
		assert!(!super::Path::program().to_str().unwrap().eq("."));
	}

	/// Smoke time
	#[test]
	fn smoke_time()
	{
		for _ in 0..1000
		{
			let l_str = super::Time::to_string(super::Time::now());
			match super::Time::from_string(l_str.as_str())
			{
				Some(m_dt) => assert_eq!(l_str, super::Time::to_string(m_dt).as_str()),
				None => assert!(false),
			};
		}
	}
}
