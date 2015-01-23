fn main() {
    let x = 1i;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("anything"),
    }

    match x {
        e @ 1 ... 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    let y = OptionalInt::Value(5i);
    match y {
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }

    match y {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }

    let z = &5i;
    match z {
        &val => println!("Got a value: {}", val),
    }

    let m = 5i;
    match m {
        ref r => println!("Got a reference to {}", r),
    }
    let mut n = 5i;
    match n {
        ref mut r => println!("Got a mutable reference to {}.", r),
    }

    let origin = Point { x: 0i, y: 0i };
    match origin {
        Point { x: x, y: y } => println!("({},{})", x, y),
    }
    match origin {
        Point { x: x, .. } => println!("x is {}", x),
    }
    match origin {
        Point { y: y, .. } => println!("y is {}", y),
    }

    let v = vec!["match_this", "1"];
    match v.as_slice() {
        ["match_this", second] => println!("The second element is {}", second),
        _ => {},
    }
}

enum OptionalInt {
    Value(int),
    Missing,
}

struct Point {
    x: int,
    y: int,
}
