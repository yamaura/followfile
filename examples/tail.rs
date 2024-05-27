use clap::Parser;
use followfile::FollowOptions;
use std::io::Write;
use tokio::io::AsyncReadExt;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    follow: bool,

    path: std::path::PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let input = tokio::fs::File::open(&args.path).await?;
    let mut input = FollowOptions::default()
        .stop_eof(!args.follow)
        .from_reader(input);

    let mut buf = vec![0; 1024];
    while let Ok(n) = input.read(&mut buf).await {
        if n == 0 {
            break;
        }
        std::io::stdout().write_all(&buf[..n])?;
    }
    Ok(())
}
