fn main() {
    // this is a statement
    let y = 6;

    // this is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    let x = plus_one(5);

    eprintln!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
