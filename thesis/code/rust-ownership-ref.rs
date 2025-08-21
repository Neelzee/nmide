fn main() {
    let name: String = "Nils".to_string();
    greeting(&name);
    // Now this is allowed
    greeting(&name);
}

fn greeting(name: &str) {
    println!("Hello, {}", name);
}
