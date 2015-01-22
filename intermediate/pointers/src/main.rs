fn main() {
    let x = 5i;
    let y = 8i;
    let z = &y;
    println!("{:p}", z);
    println!("{}", x + *z);

    let a = 5i;
    let b = &a;
    println!("{}", *b);
    println!("{:p}", b);
    println!("{}", b);

    let m = 5i;
    let n = &m;
    println!("{}", succ(n));
    println!("{}", succ(&m));

    let o = Box::new(5i);
    println!("{}", add_one(&*o));

    let list: List<int> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}", list);

    let p = Box::new(BigStruct {
        one: 1,
        two: 2,
        one_hundred: 100,
    });
    let y = Box::new(foo(p));
}

fn succ(x: &int) -> int { *x + 1 }

fn add_one(x: &int) -> int {
    *x + 1
}

#[derive(Show)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct BigStruct {
    one: int,
    two: int,
    one_hundred: int,
}

fn foo(x: Box<BigStruct>) -> BigStruct {
    return *x;
}

fn possibly_print(x: &Option<String>) {
    match *x {
        // GOOD: instead take a reference into the memory of the `Option`
        Some(ref s) => println!("{}", *s),
        None => {}
    }
}
