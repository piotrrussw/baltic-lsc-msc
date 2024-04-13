use clap::Parser;
use std::{sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
}, time::Duration};
use tokio::{sync::Semaphore, time::Instant};

#[derive(Parser)]
struct Args {
    #[clap(short = 'c', long = "count", default_value= "1000")]
    count: usize,

    #[clap(short = 'm', long = "max_conn", default_value = "1")]
    max_conn: usize,
}

#[derive(Default)]
struct Stats {
    error: AtomicUsize,
    success: AtomicUsize,
    total_time_taken: AtomicUsize,
    total_size: AtomicUsize,
}

async fn send_request(url: &str, stats: &Stats) {
    let start_time = Instant::now();
  
    let resp = match reqwest::get(url).await {
        Ok(r) => r,
        Err(_) => {
            stats.error.fetch_add(1, Ordering::Relaxed);
            return;
        }
    };

    let response_body = resp.bytes().await.unwrap_or_default();
    let response_size = response_body.len() as f64;

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time).as_millis();

    stats.success.fetch_add(1, Ordering::Relaxed);
    stats.total_time_taken.fetch_add(duration as usize, Ordering::Relaxed);
    stats.total_size.fetch_add(response_size as usize, Ordering::Relaxed);
}

#[tokio::main]
async fn main() {
    let args = Arc::new(Args::parse());
    let args_clone = Arc::clone(&args);

    let stats = Arc::new(Stats::default());

    let semaphore = Arc::new(Semaphore::new(args.max_conn));
    let semaphore_clone = Arc::clone(&semaphore); 

    let url = Arc::new("http://localhost:8080/");
    let mut handles = vec![];

    let now = Instant::now();
    for _ in 0..args.count {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let stats = stats.clone();
        let url = url.clone();

        handles.push(tokio::spawn(async move {
            send_request(&url, &stats).await;
            drop(permit);
        }));
    }

    tokio::spawn(async move {
        while semaphore_clone.available_permits() != args_clone.max_conn {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }).await.unwrap_or(());

    let total_time = now.elapsed().as_millis();
    let success = stats.success.load(Ordering::Relaxed);
    let total_size = stats.total_size.load(Ordering::Relaxed);
    let error = stats.error.load(Ordering::Relaxed);

    println!("total_time: {} total_size: {} success {} errors {}", total_time, total_size, success, error);
    let rps = args.count as u128 * 1000 / total_time;
    let average_ssr = stats.total_time_taken.load(Ordering::Relaxed) / success;

    println!("rps: {}, average_ssr: {}", rps, average_ssr);
}


// Yew SSR
// total_time: 92 total_size: 0 success 10000 errors 0
// rps: 108, average_ssr: 917
