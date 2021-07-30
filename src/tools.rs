pub use std::collections::HashMap;
pub use image::DynamicImage;
use option_flags::*;

#[allow(dead_code)]
pub mod option_flags {
	pub const OPTIONAL: u8    = 0b00000000;
	pub const REQUIRED: u8    = 0b00000001;
	pub const TAKES_VALUE: u8 = 0b00000010;
	pub const HIDDEN: u8      = 0b00000100;
	pub const ALLOW_EMPTY: u8 = 0b00001000;
	pub const MULTIPLE: u8    = 0b00010000;
	pub const DELIMITED: u8   = 0b00100000;
}

pub type RunInputs  = Vec<DynamicImage>;
pub type RunOptions = HashMap<String, Vec<String>>;
pub type RunResult  = Result<Option<DynamicImage>, String>;

#[derive(Clone, Copy)]
pub struct ToolOption<'a> {
	name: &'a str,
	help: Option<&'a str>,
	short: Option<&'a str>,
	long: Option<&'a str>,
	flags: u8,
}

#[derive(Clone, Copy)]
pub struct Tool<'a> {
	about: &'a str,
	inputs: i32,
	options: &'a[ToolOption<'a>],
}

macro_rules! tools {
	[ $($tool:ident),* $( , )? ] => {
		$( mod $tool; )*
		use clap::{ App, AppSettings, Arg, ArgMatches };

		pub fn run_tool<'a>(matches: ArgMatches<'a>) -> RunResult {
			let subcommand = matches.subcommand_name()
				.ok_or("No subcommand present".to_string())?;
			let matches = matches.subcommand_matches(subcommand)
				.unwrap();

			let inputs = if let Some(inputs) = matches.values_of("INPUTS") {
				inputs
					.map(|input| {
						image::open(input)
							.map_err(|err| err.to_string())
					})
					.collect::<Result<Vec<_>, _>>()?
			} else { vec![] };

			let opts = {
				let mut options = HashMap::new();
				for (key, value) in matches.args.clone() {
					options.insert(
						key.to_string(),
						value.vals.iter()
							.map(|value| {
								value.to_str()
									.ok_or("Bad UTF-8".to_string())
									.map(|string| string.to_string())
							})
							.collect::<Result<Vec<_>, _>>()?,
					);
				}

				options.clone()
			};

			match subcommand {
				$(
					stringify!($tool) => $tool::run(inputs, opts),
				)*
				_ => Err("Tool does not exist".into()),
			}
		}

		pub fn list_subcommands<'a>() -> &'a[&'a str] {
			&[ $( stringify!($tool), )* ]
		}

		pub fn subcommands<'a>(app: App<'a, 'a>) -> App {
			app
			$(
				.subcommand({
					let subc = App::new(stringify!($tool))
						.about($tool::TOOL.about)
						.setting(AppSettings::DisableVersion)
						.setting(AppSettings::TrailingVarArg);

					let subc = if $tool::TOOL.inputs != 0 {
						subc.arg({
							let arg = Arg::with_name("INPUTS")
								.help("List of input files")
								.required(true)
								.takes_value(true)
								.min_values($tool::TOOL.inputs.max(1) as u64);
							
							if $tool::TOOL.inputs > 0 {
								arg.max_values($tool::TOOL.inputs as u64)
							} else { arg }
						})
					} else { subc };
				
					subc.args(
						$tool::TOOL.options.iter()
							.map(|option| {
								let arg = Arg::with_name(option.name)
									.required((option.flags & REQUIRED) > 0)
									.hidden((option.flags & HIDDEN) > 0)
									.empty_values((option.flags & ALLOW_EMPTY) > 0)
									.use_delimiter((option.flags & DELIMITED) > 0)
									.multiple((option.flags & MULTIPLE) > 0)
									.takes_value((option.flags & TAKES_VALUE) > 0)
									.long(option.long.unwrap_or(option.name));
								
								let arg = if let Some(short) = option.short {
									assert_ne!(short, "o");
									arg.short(short)
								} else { arg };

								let arg = if let Some(help) = option.help {
									arg.help(help)
								} else { arg };
								
								arg
							})
							.collect::<Vec<_>>()
							.as_slice()
					)
				})
			)*
		}
	};
}

macro_rules! define_tool {
	{
		about: $about:literal,
		inputs: $inputs:expr,
		options: $options:expr $( , )?
	} => {
		#[allow(unused_imports)]
		use super::{
			Tool, ToolOption,
			RunInputs,
			RunOptions,
			RunResult,
			option_flags::*,
		};

		#[allow(dead_code)]
		#[allow(non_upper_case_globals)]
		const any: i32 = -1i32;

		pub const TOOL: Tool = Tool {
			about: $about,
			inputs: $inputs,
			options: &$options,
		};
	};
}

macro_rules! option {
	( $opts:ident, value of $key:literal ) => {
		$opts.get($key)
			.and_then(|vals| vals.get(0))
			.map(|rf| (*rf).clone())
			.unwrap()
	};

	( $opts:ident, value of $key:literal or $or:expr ) => {
		$opts.get($key)
			.and_then(|vals| vals.get(0))
			.map(|rf| (*rf).clone())
			.unwrap_or(($or).into())
	};

	( $opts:ident, value of $key:literal $( or $or:expr )? => $to:ty ) => {
		{
			use std::str::FromStr;

			<$to>::from_str(option!($opts, value of $key $( or $or )?).as_str())
				.map_err(|_| format!("Argument {} invalid", $key))
		}
	};
	
	( $opts:ident, values of $key:literal ) => {
		$opts.get($key)
			.map(|rf| (*rf).clone())
			.unwrap_or(vec![])
	};

	( $opts:ident, values of $key:literal => $to:ty ) => {
		{
			use std::str::FromStr;

			option!($opts, values of $key)
				.iter()
				.map(|s| <$to>::from_str(s.as_str()))
				.collect::<Result<Vec<_>, _>>()
				.map_err(|_| format!("Argument {} invalid", $key))
		}
	};
	
	( $opts:ident, is present $key:literal ) => {
		$opts.get($key).is_some()
	};
}

tools! [
	border,
	color,
	concat,
	convert,
	crop,
	filter,
	info,
	layer,
	pixel,
	resize,
	transform,
];
