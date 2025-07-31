// enum IpAdrrKind {
// 	V4,
// 	V6,
// }

// struct IpAddr {
// 	kind: IpAdrrKind,
// 	address: String,
// }

enum IpAdrr {
	// V4(String),
	V4(u8, u8, u8, u8),
	V6(String),
}

enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
	}
}

fn main() {
	// let four = IpAdrrKind::V4;
	// let six = IpAdrrKind::V6;

	// let home = IpAddr {
	// 	kind: IpAdrrKind::V4,
	// 	address: String::from("127.0.0.1"),
	// };

	// let loopback = IpAddr {
	// 	kind: IpAdrrKind::V6,
	// 	address: String::from("::1"),
	// };

	let home = IpAdrr::V4(127, 0, 0, 1);
	let loopback = IpAdrr::V6(String::from("::1"));

	let m = Message::Write(String::from("hello"));

	m.call();
}

// fn route(ip: IpAdrrKind) {}
