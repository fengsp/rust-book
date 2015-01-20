fn main() {
    let a = [1, 2, 2];  // a: [i32; 3]
    let mut m = [1, 2, 3];  // mut m: [i32; 3]
    let b = [0; 20]; // b: [i32; 20]

    println!("a has {} elements", a.len());
    for e in a.iter() {
        println!("{}", e);
    }

    let names = ["Graydon", "Brian", "Niko"];  // names: [&str; 3]
    println!("The second name is: {}", names[1]);

    let v = vec![1, 2, 3];  // v: Vec<i32>
    let mut nums = vec![1, 2, 3];  // mut nums: Vec<i32>
    nums.push(4);
    println!("The length of nums is now {}", nums.len());  // Prints 4

    let c = [0, 1, 2, 3, 4];
    let middle = c.slice(1, 4);  // A slice of a: just the elements [1,2,3]
    for e in middle.iter() {
        println!("{}", e);  // Prints 1, 2, 3
    }
}
