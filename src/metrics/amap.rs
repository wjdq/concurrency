use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::sync::atomic::AtomicI64;
use std::sync::Arc;

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl AmapMetrics {
    pub fn new(metrics_names: &[&'static str]) -> Self {
        let map = metrics_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();

        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> anyhow::Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("key : {} not found", key))?;
        counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (key, counter) in self.data.iter() {
            writeln!(
                f,
                "{}: {}",
                key,
                counter.load(std::sync::atomic::Ordering::Relaxed)
            )?;
        }
        Ok(())
    }
}
