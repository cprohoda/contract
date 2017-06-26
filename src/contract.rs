extern crate time;

struct contract {
	startDate: time::Tm,
	interval: time::Duration,
	contractType: ContractType,
	description: str,
	title: str,
	skills: Vec<str>,
	compensation: f32,
	owner: str,
	bids: HashMap<str,f32>,
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

type ContractResult = Result<contract, ContractError>; //

impl contract {
	fn new(user: User, startDate: time::Tm, interval: time::Duration, contractType: ContractType, description: str, title: str, skills: Vec<str>, compensation: f32,) -> contract {
		contract {
			owner: user.current(),
			startDate: startDate,
			interval: interval,
			contractType: conractType,
			description: description,
			title: title,
			skills: skills,
			compensation: compensation,
			bids: HashMap::new(),
		}
	}

	fn AddBid(&mut self, user: User, bid: bid) {
		self.bids.insert(user.current(), bid);
	}
}