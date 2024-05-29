#![clippy::pendantic]
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> io::Result<()> {
    // easy example
    let mut stream = tokio_stream::iter(vec![1, 2, 3]);

    while let Some(value) = stream.next().await {
        println!("Got: {}", value);
    }

    // file reading example
    // Paths for the source and destination files
    let src_path = "source.txt";
    let dest_path = "destination.txt";

    // Open the source file for reading
    let mut src_file = File::open(src_path).await?;

    // Open or create the destination file for writing
    let mut dest_file = File::create(dest_path).await?;

    // Create a buffer to hold the data being read
    let mut buffer = vec![0; 1024];

    // Loop to read from the source file and write to the destination file
    loop {
        // Read a chunk of data from the source file
        let bytes_read = src_file.read(&mut buffer).await?;

        // If no more data is read, break out of the loop
        if bytes_read == 0 {
            break;
        }

        // Write the chunk of data to the destination file
        dest_file.write_all(&buffer[..bytes_read]).await?;
    }

    // Ensure all data is flushed and written to the destination file
    dest_file.flush().await?;

    Ok(())
}
