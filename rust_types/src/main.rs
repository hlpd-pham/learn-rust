use std::io;

fn main() {
    let spaces = " ";
    println!("spaces variable: '{spaces}'");

    let spaces = spaces.len();
    println!("spaces variable changed to: '{spaces}'");

    let some_b: bool = true;
    let some_b = !some_b;
    println!("some_b {some_b}");

    let c = 'å';
    println!("weird char {c}");

    let tup = (500, false, "hello_world");
    let (num, some_b, some_str) = tup;
    println!("tup: {:?}, members: {num}, {some_b}, {some_str}", tup);

    array_1();
}

fn array_1() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index. array is {:?}", a);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
