use std::io;


fn main() {
    println!("Type something!");

                                                  // here, we'll show the types at each step

    let input = io::stdin()                       // std::io::stdio::StdinReader
                  .read_line()                    // IoResult<String>
                  .ok()                           // Option<String>
                  .expect("Failed to read line"); // String

    println!("{}", input);
}
