use std::env;

// fn main() {
//     let rust_env = env::var("RUST_ENV").unwrap_or("beta".to_owned());
//     println!("rust_env {} .", rust_env);
//     println!("Hello, world! {}", rust_env);
// }

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let rust_env = env::var("RUST_ENV").unwrap_or("beta".to_owned());
    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("Hello, world!") });

    let addr = "0.0.0.0:8080";
    println!("rust_env: {}, addr: {}", rust_env, addr);
    app.listen(addr).await?;
    Ok(())
}
