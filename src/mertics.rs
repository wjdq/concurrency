// metrics data structure
// 基本功能： inc/dec/snapshot

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    // impl inc dec
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
    // impl snapshot
    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let data = self
            .data
            .lock()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let snapshot = data.clone();
        Ok(snapshot)
    }
}
