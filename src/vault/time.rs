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
		match chrono::Utc.datetime_from_str(a_str, "%Y-%m-%d_%H-%M-%S_%f")
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
		return a_dt.to_string()
			.replace(" ", "_")
			.replace(".", "_")
			.replace(":", "-")
			.replace("_UTC", "")
		;
	}

}

/// Tests module
mod tests
{
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
