fn main() {
    for x in range(0, 10) {
        println!("{}", x);  // x: i32
    }

    let mut x = 5u;  // mut x: uint
    let mut done = false;  // mut done: bool
    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { done = true; }
    }

    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { break; }
    }

    for x in range(0, 10) {
        if x % 2 == 0 { continue; }
        println!("{}", x);
    }
}
