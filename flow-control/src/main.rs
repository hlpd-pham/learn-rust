fn main() {
    let condition = true;
    let number_six: i32 = 6;
    let number = if condition { 5 } else { number_six };

    println!("The value of number is: {number}");

    println!("------------------------------------");
    loop_with_label();

    println!("------------------------------------");
    println!("for with index");
    iterate_through_array();

    println!("------------------------------------");
    println!("for each");
    for_each();

    println!("------------------------------------");
    println!("for range");
    for_range();
}

fn loop_with_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn iterate_through_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("array is {:?}", a);
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_each() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
