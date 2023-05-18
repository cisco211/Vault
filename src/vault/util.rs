/// Get path of program function
///
/// Gets program path without executable filename.
pub fn get_path_program() -> std::path::PathBuf
{
	match std::env::current_exe()
	{
		Ok(path) =>
		{
			match path.parent()
			{
				Some(parent) => parent.to_path_buf(),
				_ => std::path::PathBuf::from("."),
			}
		},
		_ => std::path::PathBuf::from("."),
	}
}
