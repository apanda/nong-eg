use tokio::fs::File;
use tokio::io::{self, AsyncReadExt}; // for read_to_end()
use tokio::task::JoinSet;

async fn read_len(tag: String, fs: &[&str]) -> io::Result<(String, Vec<usize>)> {
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
    Ok((tag, v))
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut j = JoinSet::new();
    j.spawn(read_len("a".into(), &["Cargo.toml", "Cargo.lock"]));
    j.spawn(read_len("b".into(), &["hello.txt", "bye.txt"]));
    let out = j.join_all().await;
    for r in out {
        let (tag, x) = r.unwrap();
        for l in x {
            println!("{} Len {}", tag, l);
        }
    }
    Ok(())
}
