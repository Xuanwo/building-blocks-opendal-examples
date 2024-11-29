use opendal::raw::{Access, AccessorInfo, OpRead, OpStat, RpRead, RpStat};
use opendal::{EntryMode, Metadata};
use std::sync::Arc;

#[derive(Debug)]
struct CmuBackend;

impl Access for CmuBackend {
    type Reader = ();
    type Writer = ();
    type Lister = ();
    type BlockingReader = ();
    type BlockingWriter = ();
    type BlockingLister = ();

    fn info(&self) -> Arc<AccessorInfo> {
        let info = AccessorInfo::default();
        Arc::new(info)
    }

    async fn stat(&self, path: &str, args: OpStat) -> opendal::Result<RpStat> {
        let _ = (path, args);
        Ok(RpStat::new(
            Metadata::new(EntryMode::FILE).with_content_length(42),
        ))
    }

    async fn read(&self, path: &str, args: OpRead) -> opendal::Result<(RpRead, Self::Reader)> {
        let _ = (path, args);
        Ok((RpRead::default(), ()))
    }
}
