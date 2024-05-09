use std::thread;

use anyhow::Result;
use rand::Rng;

use concurrency::Metrics;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();
    println!("{:?}", metrics.snapshot());

    for i in 1..N {
        task_worker(i, metrics.clone())?;
    }
    for _ in 1..M {
        request_worker(metrics.clone())?;
    }
    loop {
        thread::sleep(std::time::Duration::from_millis(1000));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
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

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rand = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rand.gen_range(50..800)));
            let page = rand.gen_range(1..256);
            metrics.inc(format!("call.thread.worker.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}
