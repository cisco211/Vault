// Mod
mod args;
mod config;
mod rotate;
mod state;
mod task;
mod time;

// Use
use crate::vault::args::Args;
use crate::vault::config::Config;
use crate::vault::task::Task;
use crate::vault::time::Time;

/// Help
fn help(a_long: bool)
{
	println!("");
	if a_long
	{
		match Args::cmd().print_long_help()
		{
			Ok(_) => (),
			Err(_) => println!("Error: Failed to show long help text!"),
		}
	}
	else
	{
		match Args::cmd().print_help()
		{
			Ok(_) => (),
			Err(_) => println!("Error: Failed to show short help text!"),
		}
	}
}

/// Run
pub fn run() -> bool
{
	// Get arguments
	let l_args = Args::read();

	// Output sample
	if l_args.sample
	{
		println!("{}", SAMPLE);
		return true;
	}

	// Config given
	else if let Some(l_config) = l_args.config
	{
		// Load configuration
		let mut l_cfg = match Config::load(&l_config)
		{
			Some(m_cfg) => m_cfg,
			None => return false,
		};

		// Debug
		l_cfg.debug = l_args.debug;
		if l_args.debug
		{
			dbg!(&l_cfg);
		}

		// No name
		if l_cfg.name.is_empty()
		{
			println!("Error: Configuration file '{}' has no name!", l_config.display());
			return false;
		}

		// Task given
		if let Some(l_task) = l_args.task
		{
			println!("vault at {}", Time::to_string(&Time::now()));
			let l_status = Task::run(&l_cfg, l_task.as_str());
			println!("");
			return l_status;
		}

		// No task given
		else
		{
			println!("Error: No task specified!");
			help(false);
			return false;
		}
	}

	// No config given
	else
	{
		println!("Error: No configuration file specified!");
		help(false);
		return false;
	}
}

// Sample configuration
pub const SAMPLE: &str = r#"############################################
# Sample configuration file in TOML format #
############################################

# Name of this configuration.
# The name can be anything, except being empty.
# Default: (empty)
name = "test"

# A task in this configuration.
# The task is named by the string after "task.", in this case "name_of_task".
# The name can be anything, except being empty.
[tasks.name_of_task]

# Array of console commands.
# In this setting you can specify, which commands have to be executed.
# Usually you can specify backup commands here,
# but it actually doesn't matter what these commands are.
# Vault will simply execute them.
# The order of commands in this array is also the order of execution.
# Each command can also contain special macro keywords,
# that will be evaluated by Vault and replaced with dynamic data.
# The following macro keywords exist:
# {NOW} = Current time and date of the exact moment.
#         Do not use this keyword for filenames and directory names, ...,
#         because some time has passed each time it is used (See "{STAMP}").
#         Format: %Y-%m-%d_%H-%M-%S%.9f
#         Example: 2023-05-20_04-55-11.007757100
# {PATH} = Configured directory of this task (See "path" for description).
#          Example: /home/cisco211/vault/test/linux
# {STAMP} = Current time and date stamp of this task.
#           The date and time will not change while the task runs.
#           This is ideal for filenames and directory names.
#           Format: %Y-%m-%d_%H-%M-%S%.9f
#           Example: 2023-05-20_06-19-41.386912900
# Default: (empty)
commands = ["touch {STAMP}.txt", "echo {STAMP}.txt"]

# Enable or disable this task.
# Good, if you want to keep the task configuration, but never execute it.
# Default: false
enabled = false

# Task execution interval in seconds.
# With this setting you can specify, how long it takes,
# until this task can be executed again.
# The idea here is that you can configure Vault as a cronjob with
# a very short time period.
# Nevertheless, the task determines how often it can run repeatedly.
# For example,
# the cronjob can run every 5 minutes while the task runs only once an hour.
# That is, the task will run once an hour +/- 5 minutes.
# Default: 0
interval = 10

# The task execution order for this task.
# When you run all tasks,
# this setting allows you to determine the order of the tasks to be run.
# Default: 0
order = 0

# The path to the directory for this task.
# You can see this like the working directory of this task.
# Ideally, this is an absolute path.
# This path is created automatically if it does not exist.
# Vault operates in this directory,
# so make sure your backups end up in this directory.
# This setting also defines what is used for "{PATH}".
# Default: (leer)
path = "/home/cisco211/vault/test/linux"

# Use rotation.
# Activates rotation individually for hourly, daily, monthly and yearly.
# This will handle (move/delete) the older backups,
# except the last one in the time period and current hour/day/month/year.
# Example: If daily is enabled,
# all backups except today's will be reduced to the last backup of that day.
# If multiple are enabled, hourly is always reduced first, then daily,
# then monthly, then yearly.
# Default: {daily = false, hourly = false, monthly = false, yearly = false}
rotate = {daily = true, hourly = true, monthly = true, yearly = true}

# Rotate strategy being used.
# Defines what exactly happens to a file when rotation strikes.
# The following strategies are possible:
# "delete" = File will be deleted and can not be recovered.
# "move" = File will be moved into "{PATH}/moved".
#          If "{PATH}/moved" does not exist, it will be created.
# Default: "move"
rotate_strategy = "move"

# Singleton mode.
# This setting prevents more than one process from performing this task.
# For example, a task could take longer than the interval defines.
# Default: true
singleton = true
"#;
