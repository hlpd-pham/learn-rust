fn main() {
    let x = 5;
    let mut y = x;

    println!("x: {x}, y: {y}");

    y += 1;
    println!("changed x: {x}, y: {y}");

    let word = String::from("hello word");
    let first_word = first_word(&word);
    println!("word: {word}, first word: {first_word}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    let res = assert_eq!(slice, &[2, 3]);
    println!("{:?}", res);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
