mod meta;
mod tools;
mod util;

use std::io;
use clap::{ App, AppSettings, Arg };
use image::ImageFormat;

fn main() {
	let mut app = App::new(meta::NAME)
		.author(meta::AUTHOR.split(':').next().unwrap())
		.version(meta::VERSION)
		.about(meta::DESCRIPTION)
		.long_about(meta::NOTICE)
		.setting(AppSettings::ArgsNegateSubcommands)
		.setting(AppSettings::ArgRequiredElseHelp)
		.arg(
			Arg::with_name("version")
				.short("v")
				.long("version")
				.help("Show version information")
				.conflicts_with("list")
		)
		.arg(
			Arg::with_name("list")
				.short("l")
				.long("list")
				.help("List available tools")
				.conflicts_with("license")
		)
		.arg(
			Arg::with_name("license")
				.short("L")
				.long("license")
				.help("Show program license")
				.conflicts_with("version")
		)
		.arg(
			Arg::with_name("oformat")
				.long("oformat")
				.help("Override output format")
				.global(true)
				.takes_value(true)
				.required(false)
		)
		.arg(
			Arg::with_name("OUTPUT")
				.short("o")
				.long("output")
				.help("Specify output file")
				.global(true)
				.takes_value(true)
				.required(false)
		);
	
	app = tools::subcommands(app);

	let matches = app.get_matches();

	if matches.is_present("OUTPUT") && matches.subcommand_name().is_none() {
		clap::Error::with_description(
			"Output can only be specified when a subcommand is used",
			clap::ErrorKind::MissingSubcommand,
		).exit()
	}

	if matches.is_present("version") {
		return println!("{} v{}", meta::NAME, meta::VERSION)
	}

	if matches.is_present("list") {
		return println!("{}", tools::list_subcommands().join("\n"))
	}

	if matches.is_present("license") {
		return println!("{}", include_str!("../LICENSE"))
	}

	let run_result = tools::run_tool(matches.clone());

	if let Err(error) = run_result {
		clap::Error::with_description(error.as_str(), clap::ErrorKind::Io)
			.exit()
	}

	let output = run_result.unwrap();

	if output.is_none() { return }

	let output = output.unwrap();

	let format = matches.value_of("oformat")
		.map(|format| {
			match util::format_from_string(format.into()) {
				Some(format) => format,
				None => clap::Error::with_description(
					"Invalid output format",
					clap::ErrorKind::InvalidValue,
				).exit(),
			}
		});

	let save_result = if let Some(output_loc) = matches.value_of("OUTPUT") {
		match format {
			Some(format) => output.save_with_format(output_loc, format),
			None => output.save(output_loc),
		}
	} else {
		output.write_to(
			&mut io::stdout(),
			format.unwrap_or(ImageFormat::Png),
		)
	};

	if let Err(error) = save_result {
		clap::Error::with_description(
			error.to_string().as_str(),
			clap::ErrorKind::Io,
		).exit()
	}
}
