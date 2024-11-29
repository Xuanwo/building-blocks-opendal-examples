use anyhow::Result;
use futures::stream::StreamExt;
use opendal::{Entry, Operator};

pub async fn list(op: Operator) -> Result<()> {
    let _: Vec<Entry> = op.list("path/to/data/").await?;

    let _: Vec<Entry> = op
        .list_with("path/to/data/")
        .recursive(true)
        .start_after("path/to/last_file")
        .await?;

    Ok(())
}

pub async fn lister(op: Operator) -> Result<()> {
    let _lister = op.lister("path/to/data/").await?;
    let mut lister = op.lister_with("path/to/data/").recursive(true).await?;

    while let Some(e) = lister.next().await.transpose()? {
        println!("{:?}: size {}", e.path(), e.metadata().content_length());
    }

    Ok(())
}
