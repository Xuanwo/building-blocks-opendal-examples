use opendal::raw::{Access, AccessorInfo, Layer, OpStat, RpStat};
use std::sync::Arc;

#[derive(Debug)]
pub struct CmuLayer;

impl<A: Access> Layer<A> for CmuLayer {
    type LayeredAccess = HelloAccessor<A>;

    fn layer(&self, inner: A) -> Self::LayeredAccess {
        HelloAccessor { inner }
    }
}

#[derive(Debug)]
pub struct HelloAccessor<A: Access> {
    inner: A,
}

impl<A: Access> Access for HelloAccessor<A> {
    type Reader = A::Reader;
    type Writer = A::Writer;
    type Lister = A::Lister;
    type BlockingReader = A::BlockingReader;
    type BlockingWriter = A::BlockingWriter;
    type BlockingLister = A::BlockingLister;

    fn info(&self) -> Arc<AccessorInfo> {
        self.inner.info()
    }

    async fn stat(&self, path: &str, args: OpStat) -> opendal::Result<RpStat> {
        println!("Hello, CMU!");

        self.inner.stat(path, args).await
    }
}
