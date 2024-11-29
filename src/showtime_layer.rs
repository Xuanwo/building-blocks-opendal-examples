use crate::architecture_layer::CmuLayer;
use anyhow::Result;
use opendal::layers::{LoggingLayer, RetryLayer};
use opendal::Operator;

pub async fn rock(op: Operator) -> Result<Operator> {
    // opendal can retry on failure.
    let op = op.layer(RetryLayer::default());

    // opendal will print log for every operation.
    let op = op.layer(LoggingLayer::default());

    // opendal will say hello to dj everytime.
    let op = op.layer(CmuLayer);

    Ok(op)
}
