/// Path program
///
/// Gets program path without executable filename.
pub fn path_program() -> std::path::PathBuf
{
	match std::env::current_exe()
	{
		Ok(path) =>
		{
			match path.parent()
			{
				Some(parent) => return parent.to_path_buf(),
				None => return std::path::PathBuf::from("."),
			}
		},
		Err(_e) => return std::path::PathBuf::from("."),
	}
}

/// Time from string
///
/// Get given string as time.
pub fn time_from_string(str: &str) -> Option<chrono::DateTime<chrono::Utc>>
{
	use chrono::TimeZone;
	match chrono::Utc.datetime_from_str(str, "%Y-%m-%d_%H:%M:%S%.f")
	{
		Ok(dt) => return Some(dt),
		Err(_e) => return None,
	}
}

/// Time now
///
/// Get current time.
pub fn time_now() -> chrono::DateTime<chrono::Utc>
{
	return chrono::offset::Utc::now();
}

/// Time to string
///
/// Get given time as string.
pub fn time_to_string(dt: chrono::DateTime<chrono::Utc>) -> String
{
	return dt.to_string().replace(" ", "_").replace("_UTC", "");
}

/// Tests module
mod tests
{
	/// Smoke time
	///
	/// Smoke test time functions.
	#[test]
	fn smoke_time()
	{
		use crate::vault::util;
		for _i in 0..1000
		{
			let str = util::time_to_string(util::time_now());
			match util::time_from_string(str.as_str())
			{
				Some(dt) => assert_eq!(str, util::time_to_string(dt).as_str()),
				None => assert!(false),
			};
		}
	}
}
