// Use
use chrono::{DateTime, TimeZone, Utc};

/// Format
pub const FORMAT: &str = "%Y-%m-%d_%H-%M-%S%.9f";

/// Time struct
pub struct Time;

/// Time impl
impl Time
{
	/// from string
	pub fn from_string(a_str: &str) -> Option<DateTime<Utc>>
	{
		match Utc.datetime_from_str(a_str, FORMAT)
		{
			Ok(m_dt) => return Some(m_dt),
			Err(_m_error) => return None,
		}
	}

	/// Now
	pub fn now() -> DateTime<Utc>
	{
		return Utc::now();
	}

	/// To string
	pub fn to_string(a_dt: &DateTime<Utc>) -> String
	{
		return a_dt.format(FORMAT).to_string();
	}

}

/// Tests mod
mod tests
{
	/// Smoke
	#[test]
	fn smoke()
	{
		use crate::vault::time::Time as Time;
		for _ in 0..1000
		{
			let l_str = Time::to_string(&Time::now());
			match Time::from_string(l_str.as_str())
			{
				Some(m_dt) => assert_eq!(l_str, Time::to_string(&m_dt).as_str()),
				None => assert!(false),
			};
		}
	}
}
