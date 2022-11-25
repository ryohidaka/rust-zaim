mod types;
mod zaim;

#[tokio::main]
async fn main() {
    // Fetch Me
    let me = zaim::fetch_me().await.me;
    println!("{:?}", me);
}
