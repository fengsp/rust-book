fn main() {
    // Line comments are anything after '//' and extend to the end of the line.

    let x = 5;  // this is also a line comment.

    // If you have a long explanation for something, you can put line comments next
    // to each other.  Put a space between the /// and your comment so that it's
    // more readable.
}

/// `hello` is a function that prints a greeting that is personalized based on
/// the name given.
///
/// # Arguments
///
/// * `name` - The name of the person you'd like to greet.
///
/// # Example
///
/// ```rust
/// let name = "Steve";
/// hello(name);  // prints "Hello, Steve!"
/// ```
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
