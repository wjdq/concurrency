use std::thread;

use anyhow::Result;
use rand::Rng;

use concurrency::CmapMetrics;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = CmapMetrics::new();
    println!("{}", metrics);

    for i in 0..N {
        task_worker(i, metrics.clone())?;
    }
    for _ in 0..M {
        request_worker(metrics.clone())?;
    }
    loop {
        thread::sleep(std::time::Duration::from_millis(1000));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rand = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rand.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rand = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rand.gen_range(50..800)));
            let page = rand.gen_range(1..256);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
