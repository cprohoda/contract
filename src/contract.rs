extern crate time;

struct contract {
	startDate: time::Tm,
	interval: time::Duration,
	type: ContractType,
	description: str,
	title: str,
	skills: Vec<str>,
	compensation: f32,
	owner: str,
}

enum ContractType {
	recurring,
	single,
}

enum ContractError {
	authentication,
	authorization,
	value,
}

type ContractResult = Result<,ContractError>; //

impl contract {
	pub fn new() -> contract {

	}
}