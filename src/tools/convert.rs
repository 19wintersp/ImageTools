define_tool! {
	about: "Converts input through output",
	inputs: 1,
	options: [],
}

pub fn run(inputs: RunInputs, _options: RunOptions) -> RunResult {
	Ok(Some(inputs[0].clone()))
}
