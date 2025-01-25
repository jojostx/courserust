use std::io;

fn main() {
    println!("Hello, world!");

    let mut prompt = String::new();

    while prompt.trim() != "stop" {
        prompt.clear();
        println!("please enter a word, or type 'stop' to exit");
        io::stdin().read_line(&mut prompt).expect("Failed to read line");
        println!("You typed: {}", prompt);
    }

    println!("Goodbye!");
}
