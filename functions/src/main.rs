fn main() {
    let x = 1;
    let y = 2;
    println!("x: {x}, y: {y}");
    println!("Add two: {:?}", add_two(x, y));
}

fn add_two(x: i32, y: i32) -> i32 {
    x + y
}
