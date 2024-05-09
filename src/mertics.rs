// metrics data structure
// 基本功能： inc/dec/snapshot

use std::fmt;
use std::fmt::Formatter;
use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }
    // impl inc dec
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            #[allow(clippy::write_with_newline)]
            write!(f, "{}: {}\n", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
