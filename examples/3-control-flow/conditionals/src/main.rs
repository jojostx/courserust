fn cond(){
    let test = true;

    if test {
        println!("Test is true");
    } else {
        println!("Test is false");
    }
}

fn iflet(){
    let mut maybe_number = Some(42);

    match maybe_number {
        Some(ref mut val) => *val += 10,
        None => (),
    }

    if let Some(number) = maybe_number {
        println!("Number: {}", number);
    } else {
        println!("No number");
    }
}

fn main() {
    cond();
    iflet();
    println!("Hello, world!");
}
