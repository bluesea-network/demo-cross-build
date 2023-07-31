#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let sigit = tokio::signal::ctrl_c().await;
    println!("sigit: {:?}", sigit);
}
