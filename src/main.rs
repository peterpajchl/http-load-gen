mod input_params;

use clap::Parser;
use http_load_gen_lib::connection_task::{run, RunSettings};
use http_load_gen_lib::notification::{NotificationTask};
use input_params::InputParams;
use tokio::sync::mpsc::UnboundedReceiver;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use tokio::time::Duration;

#[tokio::main]
async fn main() {

    let cli = InputParams::parse();
    let target_url = cli.target_url.clone();
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<NotificationTask>();
    let actual_requests = cli.requests / cli.connections as u64 * cli.connections as u64;  
    let run_settings = RunSettings {
        requests: cli.requests,
        connections: cli.connections,
        target_url: cli.target_url
    };

    let bar = ProgressBar::new(actual_requests);
    println!("Executing run against: {}", target_url);

    let notif = tokio::spawn(get_updates(bar.clone(), rx));
    let forced_update = tokio::spawn(force_bar_update_at_interval(bar, Duration::from_millis(50)));
    let summary = run(run_settings, Some(tx)).await.expect("Execution failed");

    let _ = notif.await;
    forced_update.abort();

    let mut total_success_requests: u64 = 0;
    let mut total_fail_requests: u64= 0;

    let total_requests: u64 = summary.stats_summaries.iter().map(|stat| {
        total_success_requests += stat.requests_succeded;
        total_fail_requests += stat.requests_failed;
        stat.requests_succeded + stat.requests_failed
    }).sum();

    println!("======================================================================");
    println!("Con.: {} Req.: {total_requests} Success: {total_success_requests} Failed: {total_fail_requests} Elapsed: {} ms Rate: {} req/s", cli.connections, summary.total_duration_ms, total_success_requests * 1000 / summary.total_duration_ms);

}

async fn get_updates(bar: ProgressBar, mut rx: UnboundedReceiver<NotificationTask>) {

    let mut progress: HashMap<usize, u64> = HashMap::new();
    
    bar.set_prefix("Benchmarking");
    bar.set_style(ProgressStyle::with_template("[{spinner:.green}] [{elapsed:>3}] {prefix:.green} [{bar:40}] {human_len:7} @ {per_sec:.green}")
        .unwrap()
        .progress_chars("=> ")
    );

    while let Some(ref x) = rx.recv().await {
        progress.insert(x.connection_id as usize, x.progress_count);
        bar.set_position(progress.iter().map(|x| x.1).sum());
    }

    bar.finish_and_clear();
}

async fn force_bar_update_at_interval(bar: ProgressBar, interval: Duration) {
    let mut interval = tokio::time::interval(interval);
    
    loop {
        bar.tick();
        interval.tick().await;
    }
}