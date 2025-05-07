use tokio::fs::File;
use tokio::io::{self, AsyncReadExt}; // for read_to_end()

async fn read_len(fs: &[&str]) -> io::Result<Vec<usize>> {
    let mut v = vec![];
    for f in fs {
        v.push(
            async {
                println!("Reading {}", f);
                let mut file = File::open(f).await?;

                let mut contents = vec![];
                file.read_to_end(&mut contents).await?;
                println!("Done Reading {}", f);
                Ok::<usize, io::Error>(contents.len())
            }
            .await?,
        )
    }
    Ok(v)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let x = read_len(&["Cargo.toml", "Cargo.lock"]).await?;
    for l in x {
        println!("Len {}", l);
    }
    Ok(())
}
