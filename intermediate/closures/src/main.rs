fn main() {
    let add_one = |&: x| { 1 + x };
    println!("The sum of 5 plus 1 is {}.", add_one(5));

    let x: i32 = 5;
    let printer = |&:| { println!("x is: {}", x); };
    printer();

    let square = |&: x: i32| { x * x };
    twice(5, square);

    twice(5, |x: i32| { x * x });
}

fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    f(x) + f(x)
}
