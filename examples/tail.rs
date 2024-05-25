use std::io::Write;
use followfile::File;
    use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let input = tokio::fs::File::open(std::env::args().nth(1).unwrap()).await?;
    let mut input = File::from_reader(input);

    let mut buf = vec![0; 1024];
    while let Ok(n) = input.read(&mut buf).await {
        std::io::stdout().write_all(&buf[..n])?;
    }
    Ok(())
}
