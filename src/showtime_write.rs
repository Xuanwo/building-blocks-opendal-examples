use anyhow::Result;
use opendal::Operator;

pub async fn write_1(op: Operator) -> Result<()> {
    let bs = "Hello, DJ!";

    // write all content at once.
    op.write("path/to/data", bs).await?;

    // append all content at once.
    op.write_with("path/to/data", bs).append(true).await?;

    Ok(())
}

pub async fn write_2(op: Operator) -> Result<()> {
    let bs = "Hello, DJ!";

    // write content with extra metadata
    op.write_with("path/to/data", bs)
        .cache_control("max-age=604800")
        .content_type("text/plain")
        .content_disposition("attachment; filename=\"cool.txt\"")
        .user_metadata([
            ("dj".to_string(), "cool".to_string()),
            ("style".to_string(), "hiphop".to_string()),
        ])
        .await?;

    Ok(())
}

pub async fn write_3(op: Operator) -> Result<()> {
    let bs = "Hello, DJ!";

    // write content concurrently
    op.write_with("path/to/data", bs)
        .chunk(8 * 1024 * 1024)
        .concurrent(8)
        .await?;

    Ok(())
}

pub async fn writer_1(op: Operator) -> Result<()> {
    let bs = "Hello, DJ!";

    let _w = op.writer("path/to/data").await?;
    let mut w = op
        .writer_with("path/to/data")
        .chunk(8 * 1024 * 1024)
        .concurrent(8)
        .await?;

    w.write(bs).await?;
    w.write(bs).await?;
    w.close().await?;
    Ok(())
}

pub async fn writer_2(op: Operator) -> Result<()> {
    let w = op.writer("path/to/data").await?;

    // convert into futures::AsyncWrite
    let _ = w.into_futures_async_write();

    let w = op.writer("path/to/data").await?;

    // convert into futures::Sink
    let _ = w.into_bytes_sink();
    Ok(())
}
