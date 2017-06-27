use std:Collections::HashMap;
// database dependency

struct User {
	username: str;
	displayName: str;
	balance: f32;
	history: Vec<str>;
	skills: HashMap<str, u32>;
	verified: bool;
	permissions: PermissionLevel;
}

enum PermissionLevel {
	limited,
	full,
	admin,
}

impl User {
	pub fn new(username: str, displayName: str, permissions: PermissionLevel) -> user {
		// read from DB
		User { //temporary init until DB added
			username: username,
			displayName: displayName,
			balance: 0f32,
			history: Vec::new(),
			skills: HashMap::new(),
			permissions: permissions,
			verified: true,
		}
	}

	pub fn ChangeDisplayName(&mut self, newName: str) {
		self.displayName = newName;
	}

	pub fn ChangePassword(&mut self, oldPassword: str, newPassword: str) {
		// verify password here
		// hash and salt and shit
		self.password = newPassword;
		// add to database
	}

	pub fn ViewHistory(self) -> Vec<str> {
		self.history
	}

	pub fn ViewSkills(self) -> Vec<str> {
		self.skills
	}

	pub fn AddSkill(&mut self, skill: str) {
		self.skills.insert(skill, 0u32);
	}

	pub fn Current(self) -> str {
		self.username
	}

	pub fn PostContract(&mut self, ) -> ContractResult {
		contract::new(self.Current, )
	}

	enum BidError {
		lowBalance,
		authorization,
		authentication,
	}

	type BidResult = Result<(), BidError>;

	pub fn BidContract(&mut self, contract: contract, bid: f32) -> BidResult {
		if balance >= bid {
			self.balance -= bid;
			contract.AddBid(self, bid);
			Ok(())
		} else {
			Err(BidError::lowBalance)
		}
	}
}

pub fn login(username, password) -> User {
	// verify password here
	User::new(username)
}
