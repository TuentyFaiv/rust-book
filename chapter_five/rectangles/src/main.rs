#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn square(size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}

	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn width(&self) -> bool {
		self.width > 0
	}

	fn can_hold(&self, child: &Rectangle) -> bool {
		self.width > child.width && self.height > child.height
	}
}

fn main() {
	// basic rust
	// let width1 = 30;
	// let height1 = 50;

	// println!("The area of the rectangle is {} square pixels.", area(width1, height1));

	// using tuples
	// let rect1 = (30, 50);

	// println!("The area of the rectangle is {} square pixels.", area(rect1));

	// using structs
	let scale = 2;
	let rect1 = Rectangle {
		// width: dbg!(30 * scale),
		width: 30,
		height: 50,
	};
	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};
	let rect3 = Rectangle {
		width: 60,
		height: 45,
	};

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

	let sq = Rectangle::square(3);

	// dbg!(&rect1);
	// println!("rect1 is {:#?}", rect1);

	// println!("The area of the rectangle is {} square pixels.", area(&rect1));
	// println!("The area of the rectangle is {} square pixels.", rect1.area());
	// if rect1.width() {
	// 	println!("The rectangle has a nonzero width; it is {}", rect1.width);
	// }
	// println!("The area of the rectangle is {} square pixels.", rect1.area());
}



// fn area(width: u32, height: u32) -> u32 {
// fn area(dimensions: (u32, u32)) -> u32 {
// fn area(rectangle: &Rectangle) -> u32 {
// 	rectangle.width * rectangle.height
// }
