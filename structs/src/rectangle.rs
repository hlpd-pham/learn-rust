#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn debug_rectangle() {
    println!("--------------------");
    println!("debug a rectangle");
    let rec_width = 10;
    let rec_height = 5;
    let rec = Rectangle {
        width: dbg!(rec_width * 2),
        height: rec_height,
    };
    dbg!(&rec);
    println!("--------------------");
}

pub fn rec_fn() {
    let r1 = Rectangle {
        width: 10,
        height: 50,
    };

    println!("The area of the retangle is {} square pixels", r1.area());

    println!("rectangle info: {:?}", r1);

    println!("rectangle info pretty print: {:#?}", r1);

    debug_rectangle();

    let r2 = Rectangle {
        width: 5,
        height: 2,
    };
    let r3 = Rectangle {
        width: 11,
        height: 20,
    };

    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);
    println!("r3: {:?}", r3);

    println!("can r1 hold r2? {}", r1.can_hold(&r2));
    println!("can r1 hold r3? {}", r1.can_hold(&r3));
    println!("can r2 hold r3? {}", r2.can_hold(&r3));
    println!("--------------------");

    let square = Rectangle::square(5);
    println!("a square from rectangle: {:?}", square);
}
