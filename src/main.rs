mod call_endpoint;

#[tokio::main]
async fn main() {
    println!("{}", call_endpoint::get_inspiring_quote().await.unwrap())
}
