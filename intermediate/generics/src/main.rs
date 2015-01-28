fn main() {
    let x: Option<int> = Some(5i);
    let y: Option<f64> = Some(5.0f64);
    let a: Result<f64, String> = Ok(2.3f64);
    let b: Result<f64, String> = Err("There was an error.".to_string());

    let z = inverse(25.0f64);
    match z {
        Ok(x) => println!("The inverse of 25 is {}", x),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn inverse64(x: f64) -> Result<f64, String> {
    if x == 0.0f64 { return Err("x cannot be zero!".to_string()); }

    Ok(1.0f64 / x)
}

fn inverse32(x: f32) -> Result<f32, String> {
    if x == 0.0f32 { return Err("x cannot be zero!".to_string()); }

    Ok(1.0f32 / x)
}

fn inverse<T>(x: T) -> Result<T, String> {
    if x == 0.0 { return Err("x cannot be zero!".to_string()); }

    Ok(1.0 / x)
}
