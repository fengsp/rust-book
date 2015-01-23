use std::rc::Rc;

fn main() {
    let x = Box::new(5i);

    let y = add_one(x);

    println!("{}", y);

    let z = &5i;
    let f = Foo { x: z };
    println!("{}", f.x);

    let a: &'static str = "Hello, world.";
    static FOO: int = 5i;
    let b: &'static int = &FOO;

    let car = Car { name: "Delorean".to_string() };
    let car_owner = Rc::new(car);
    for _ in range(0u, 4) {
        Wheel { size: 360, owner: car_owner.clone() };
    }
}

fn add_one(mut num: Box<int>) -> Box<int> {
    *num += 1;
    num
}

struct Foo<'a> {
    x: &'a int,
}

struct Car {
    name: String,
}

struct Wheel {
    size: int,
    owner: Rc<Car>,
}
