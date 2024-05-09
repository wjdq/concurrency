// metrics data structure
// 基本功能： inc/dec/snapshot

use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};

use anyhow::Result;

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    // impl inc dec
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
    // impl snapshot
    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let data = self
            .data
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let snapshot = data.clone();
        Ok(snapshot)
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = self.data.read().map_err(|_e| fmt::Error)?;

        for (k, v) in data.iter() {
            #[allow(clippy::write_with_newline)]
            write!(f, "{}: {}\n", k, v)?;
        }
        Ok(())
    }
}
