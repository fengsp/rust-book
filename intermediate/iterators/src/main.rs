fn main() {
    let mut r = range(0i, 10i);
    loop {
        match r.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => { break }
        }
    }

    let nums = vec![1i, 2i, 3i];
    for num in nums.iter() {
        println!("{}", num);
    }

    let one_to_one_hundred = range(1i, 101i).collect::<Vec<int>>();
    let greater_than_forty_two = range(0i, 100i).find(|x| *x > 42);
    match greater_than_forty_two {
        Some(_) => println!("We got some numbers!"),
        None => println!("No numbers found :("),
    }
    let sum = range(1i, 4i).fold(0i, |sum, x| sum + x);

    std::iter::count(1i, 5i);

    range(1i, 100i).map(|x| x + 1i);
    range(1i, 100i).map(|x| println!("{}", x));
    for i in std::iter::count(1i, 5i).take(5) {
        println!("{}", i);
    }
    for i in range(1i, 100i).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }
    range(1i, 1000i)
        .filter(|&x| x % 2 == 0)
        .filter(|&x| x % 3 == 0)
        .take(5)
        .collect::<Vec<int>>();
}
