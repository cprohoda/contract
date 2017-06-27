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
	bids: Vec<Bid>,
	filters: Vec<str>,
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

struct Bid {
	bidder: str,
	skills: HashMap<str,u32>,
	compensation: f32,
	history: Vec<str>,
}

enum Filter {
	ContractType,
}

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
			bids: Vec::new(),
		}
	}

	fn AddBid(&mut self, user: User, bid: bid) {
		self.bids.insert(user.current(), bid);
	}

	fn ReviewBids(self, )
}