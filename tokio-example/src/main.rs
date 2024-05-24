use mini_redis::client;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

/// tokio::main ist ein wrapper, der die tokio runtime erstellt
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?; //Errortyp Datenbankconnection

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?; // errortyp

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    let file = File::open("test").await?;
    let mut lines = BufReader::new(file).lines();
    while let Some(line) = lines.next_line().await? {
        println!("{line}");
    }

    Ok(())
}
