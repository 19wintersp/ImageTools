define_tool! {
	about: "Applies filters to an image",
	inputs: 1,
	options: [
		ToolOption {
			name: "contrast",
			long: Some("contrast"),
			short: Some("c"),
			help: Some("Adjust contrast"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "blur",
			long: Some("blur"),
			short: Some("B"),
			help: Some("Apply gaussian blur"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "brighten",
			long: Some("brighten"),
			short: Some("b"),
			help: Some("Brighten image"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "filter",
			long: Some("filter"),
			short: Some("f"),
			help: Some("Apply 3x3 filter"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "rotate",
			long: Some("hue-rotate"),
			short: Some("h"),
			help: Some("Rotate hue"),
			flags: OPTIONAL | TAKES_VALUE,
		},
		ToolOption {
			name: "invert",
			long: Some("invert"),
			short: Some("i"),
			help: Some("Invert image"),
			flags: OPTIONAL,
		},
		ToolOption {
			name: "grayscale",
			long: Some("grayscale"),
			short: Some("g"),
			help: Some("Make image grayscale"),
			flags: OPTIONAL,
		},
	],
}

pub fn run(inputs: RunInputs, options: RunOptions) -> RunResult {
	let mut image = inputs[0].clone();

	if option!(options, is present "contrast") {
		image = image.adjust_contrast(
			option!(options, value of "contrast" => f32)?
		);
	}

	if option!(options, is present "blur") {
		image = image.blur(
			option!(options, value of "blur" => f32)?
		);
	}

	if option!(options, is present "brighten") {
		image = image.brighten(
			option!(options, value of "brighten" => i32)?
		);
	}

	if option!(options, is present "rotate") {
		image = image.huerotate(
			option!(options, value of "rotate" => i32)?
		);
	}

	if option!(options, is present "invert") {
		image.invert();
	}

	if option!(options, is present "grayscale") {
		image = image.grayscale();
	}

	if option!(options, is present "filter") {
		use std::str::FromStr;

		let filter = option!(options, value of "filter")
			.split(":")
			.map(|s| f32::from_str(s))
			.collect::<Result<Vec<_>, _>>()
			.map_err(|_| "Argument filter invalid".to_string())?;
		
		if filter.len() != 9 {
			return Err("Filter must be 9 colon-separated floats".into())
		}

		image = image.filter3x3(filter.as_slice());
	}

	Ok(Some(image))
}
