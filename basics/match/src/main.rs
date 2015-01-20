use std::cmp::Ordering;


fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

enum OptionalInt {
    Value(i32),
    Missing,
}

fn main() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let y = 10;
    match cmp(x, y) {
        Ordering::Less => println!("less"),
        Ordering::Greater => println!("greater"),
        Ordering::Equal => println!("equal"),
    }

    let a = OptionalInt::Value(5);
    let b = OptionalInt::Missing;
    match a {
        OptionalInt::Value(n) => println!("a is {}", n),
        OptionalInt::Missing => println!("a is missing!"),
    }
    match b {
        OptionalInt::Value(n) => println!("b is {}", n),
        OptionalInt::Missing => println!("b is missing!"),
    }

    println!("{}", match cmp(x, y) {
        Ordering::Less => "less",
        Ordering::Greater => "greater",
        Ordering::Equal => "equal",
    });
}
