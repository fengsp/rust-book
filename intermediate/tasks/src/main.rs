fn main() {
    // Print something profound in a different thread using a named function
    fn print_message() { println!("I am running in a different thread!"); }
    spawn(print_message);

    // Alternatively, use a `move ||` expression instead of a named function.
    // `||` expressions evaluate to an unnamed closure. The `move` keyword
    // indicates that the closure should take ownership of any variables it
    // touches.
    spawn(move || println!("I am also running in a different thread!"));
}
