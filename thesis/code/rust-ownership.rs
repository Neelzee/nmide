fn main() {
    let name: String = "Nils".to_string();
    greeting(name);
    // Not allowed
    greeting(name);
}

fn greeting(name: String) {
    println!("Hello, {}", name);
}
