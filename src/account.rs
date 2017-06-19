use std:Collections::HashMap;
// database dependency

struct User {
	username: str;
	displayName: str;
	balance: f32;
	history: Vec<str>;
	skills: HashMap<&str, u32>;
	verified: bool;
	permissions: PermissionLevel;
}

enum PermissionLevel {
	limited,
	full,
	admin,
}

impl User {
	fn new(username: &str) -> user {
		// read from DB
	}

	pub fn ChangeDisplayName(&mut self, newName: str) {
		self.displayName = newName;
	}

	pub fn ChangePassword(&mut self, oldPassword: str, newPassword: str) {
		// verify password here
		self.password = newPassword;
		// add to database
	}

	pub fn ViewHistory(self) {
		self.history
	}

	pub fn ViewSkills(self) {
		self.skills
	}

	pub fn ReturnRelevantSkills(self, relevantSkills: Vec<str>) {

	}

	pub fn PostContract(self, )
}

pub fn login(username, password) -> user {
	// verify password here
	User.new(&username)

}
