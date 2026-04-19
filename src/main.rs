fn main() {
    println!("WOM system booting...");
    greet("Serge")    
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}