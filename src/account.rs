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
		self.password = newPassword;
		// add to database
	}

	pub fn ViewHistory(self) -> Vec<str> {
		self.history
	}

	pub fn ViewSkills(self) -> Vec<str> {
		self.skills
	}

	// pub fn ReturnRelevantSkills(self, relevantSkills: Vec<str>) {

	// }

	pub fn PostContract(self, )
}

pub fn login(username, password) -> User {
	// verify password here
	User::new(username)
}
