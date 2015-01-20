fn main() {
    let string = "Hello there.";
    let mut s = "Hello".to_string();
    println!("{}", s);
    s.push_str(", world.");
    println!("{}", s);

    takes_slice(s.as_slice());
}

fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}

fn compare(string: String) {
    if string.as_slice() == "Hello" {
        println!("yes");
    }
}
