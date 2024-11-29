use opendal::raw::{Access, AccessorInfo, OpRead, OpStat, RpRead, RpStat};
use opendal::{Builder, Capability, Configurator, EntryMode, Metadata, Scheme};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CmuConfig {
    pub dj: String,
}

impl Configurator for CmuConfig {
    type Builder = CmuBuilder;

    fn into_builder(self) -> Self::Builder {
        CmuBuilder { cfg: self }
    }
}

#[derive(Debug, Default)]
pub struct CmuBuilder {
    cfg: CmuConfig,
}

impl Builder for CmuBuilder {
    const SCHEME: Scheme = Scheme::Custom("cmu");
    type Config = CmuConfig;

    fn build(self) -> opendal::Result<impl Access> {
        Ok(CmuService { dj: self.cfg.dj })
    }
}

#[derive(Debug)]
pub struct CmuService {
    dj: String,
}

impl Access for CmuService {
    type Reader = ();
    type Writer = ();
    type Lister = ();
    type BlockingReader = ();
    type BlockingWriter = ();
    type BlockingLister = ();

    fn info(&self) -> Arc<AccessorInfo> {
        let mut info = AccessorInfo::default();
        info.set_name(&self.dj);
        info.set_native_capability(Capability {
            stat: true,
            ..Default::default()
        });

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
