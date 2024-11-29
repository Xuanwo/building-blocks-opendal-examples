use anyhow::Result;
use opendal::Operator;

pub async fn read_1(op: Operator) -> Result<()> {
    // read all content at once.
    let _ = op.read("path/to/data").await?;

    // read specific range of content.
    let _ = op.read_with("path/to/data").range(1024..2048).await?;

    Ok(())
}

pub async fn read_2(op: Operator) -> Result<()> {
    // read content in chunks.
    let _ = op.read_with("path/to/data").chunk(1024 * 1024).await?;
    // read content in chunks concurrently.
    let _ = op
        .read_with("path/to/data")
        .chunk(1024 * 1024)
        .concurrent(8)
        .await?;

    Ok(())
}

pub async fn read_3(op: Operator) -> Result<()> {
    // read specific version of content.
    let _ = op.read_with("path/to/data").version("version").await?;

    // read with condition.
    let _ = op.read_with("path/to/data").if_match("etag").await?;
    let _ = op.read_with("path/to/data").if_none_match("etag").await?;

    Ok(())
}

pub async fn reader(op: Operator) -> Result<()> {
    let _r = op.reader("path/to/data").await?;
    let r = op
        .reader_with("path/to/data")
        .chunk(1024 * 1024)
        .concurrent(8)
        .await?;

    // read all
    let _ = r.read(..).await?;
    // read range.
    let _ = r.read(1024..2048).await?;
    // read multiple ranges.
    let _ = r.fetch(vec![1024..2048, 2049..5120]).await?;

    // into futures::AsyncRead.
    let _ = r.clone().into_futures_async_read(0..4096).await?;

    // into futures::Stream.
    let _ = r.into_bytes_stream(0..4096).await?;

    Ok(())
}
