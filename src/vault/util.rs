/// Path program
pub fn path_program() -> std::path::PathBuf
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

/// Time from string
pub fn time_from_string(a_str: &str) -> Option<chrono::DateTime<chrono::Utc>>
{
	use chrono::TimeZone;
	match chrono::Utc.datetime_from_str(a_str, "%Y-%m-%d_%H:%M:%S%.f")
	{
		Ok(m_dt) => return Some(m_dt),
		Err(_m_error) => return None,
	}
}

/// Time now
pub fn time_now() -> chrono::DateTime<chrono::Utc>
{
	return chrono::offset::Utc::now();
}

/// Time to string
pub fn time_to_string(a_dt: chrono::DateTime<chrono::Utc>) -> String
{
	return a_dt.to_string().replace(" ", "_").replace("_UTC", "");
}

/// Tests module
mod tests
{
	/// Smoke path
	#[test]
	fn smoke_path()
	{
		use crate::vault::util;
		assert!(!util::path_program().to_str().unwrap().eq("."));
	}

	/// Smoke time
	#[test]
	fn smoke_time()
	{
		use crate::vault::util;
		for _i in 0..1000
		{
			let str = util::time_to_string(util::time_now());
			match util::time_from_string(str.as_str())
			{
				Some(m_dt) => assert_eq!(str, util::time_to_string(m_dt).as_str()),
				None => assert!(false),
			};
		}
	}
}
