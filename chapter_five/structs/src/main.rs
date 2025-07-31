struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs
struct AlwaysEqual;

fn main() {
	// let user1 = User {
	// 	active: true,
	// 	username: String::from("someusername123"),
	// 	email: String::from("someone@example.com"),
	// 	sign_in_count: 1,
	// };
	// let mut user1 = User {
	// 	active: true,
	// 	username: String::from("someusername123"),
	// 	email: String::from("someone@example.com"),
	// 	sign_in_count: 1,
	// };
	let mut user1 = build_user(
		String::from("someusername123"),
		String::from("someone@example.com"),
	);

	// let user2 = User {
	// 	active: user1.active,
	// 	username: user1.username,
	// 	email: String::from("another@example.com"),
	// 	sign_in_count: user1.sign_in_count,
	// };

	// struct update syntax
	let user2 = User {
		email: String::from("another@example.com"),
		..user1
	};

	user1.email = String::from("anotheremail@example.com");

	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

	let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
	User {
		active: true,
		username,
		email,
		sign_in_count: 1,
	}
}
