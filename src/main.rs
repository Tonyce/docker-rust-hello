use std::env;

fn main() {
    let rust_env = env::var("RUST_ENV").unwrap_or("beta".to_owned());
    println!("rust_env {} .", rust_env);
    println!("Hello, world! {}", rust_env);
}
