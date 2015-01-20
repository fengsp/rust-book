use std::cmp::Ordering;

use StringResult::StringOK;
use StringResult::ErrorReason;


fn main() {
    let x = (1, "hello");
    let y: (i32, &str) = (1, "hello");

    let (a, b, c) = (1, 2, 3);
    println!("a is {}", a);
    
    let mut i = (1, 2);
    let j = (2, 3);
    i = j;
    
    let m = (1, 2, 3);
    let n = (2, 2, 4);
    if m == n {
        println!("yes");
    } else {
        println!("no");
    }

    let (u, v) = next_two(5);
    println!("u, v = {}, {}", u, v);

    let origin = Point { x: 0, y: 0 };  // origin: Point
    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut point = Point { x: 0, y: 0 };
    point.x = 5;
    println!("The point is at ({}, {})", point.x, point.y);

    let black = Color(0, 0, 0);
    let o = D3Point(0, 0, 0);
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);

    let ordering = cmp(5, 10);  // ordering: Ordering
    if ordering == Ordering::Less {
        println!("less");
    } else if ordering == Ordering::Greater {
        println!("greater");
    } else if ordering == Ordering::Equal {
        println!("equal");
    }
}

fn next_two(x: i32) -> (i32, i32) { (x + 1, x + 2) }

struct Point {
    x: i32,
    y: i32,
}

struct Color(i32, i32, i32);
struct D3Point(i32, i32, i32);
struct Inches(i32);

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

enum OptionalInt {
    Value(i32),
    Missing,
}

enum OptionalColor {
    Color(i32, i32, i32),
    Missing,
}

enum StringResult {
    StringOK(String),
    ErrorReason(String),
}

fn respond(greeting: &str) -> StringResult {
    if greeting == "Hello" {
        StringOK("Good morning!".to_string())
    } else {
        ErrorReason("I didn't understand you!".to_string())
    }
}
