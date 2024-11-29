use crate::architecture_layer::CmuLayer;
use crate::architecture_service::CmuConfig;
use opendal::{Buffer, Operator};

pub async fn rock() -> anyhow::Result<()> {
    let cfg = CmuConfig {
        dj: "sorry, no dj today".to_string(),
    };
    let op = Operator::from_config(cfg)?.layer(CmuLayer).finish();

    // write
    op.write("path/to/db", "hello dj".to_string()).await?;

    // read
    let _: Buffer = op.read("path/to/db").await?;

    // metadata
    let meta = op.stat("path/to/db").await?;
    println!("size: {}", meta.content_length());
    println!("etag: {:?}", meta.etag());

    Ok(())
}
