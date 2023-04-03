use clap::Parser;
use http_load_gen::connection_task::{run};
use http_load_gen::input_params::InputParams;

#[tokio::main]
async fn main() {

    let cli = InputParams::parse();
    let target_url = cli.target_url.clone();
    let summary = run(cli).await.expect("Execution failed");

    println!("Executing run against: {}", target_url);

    let mut total_success_requests: u64 = 0;
    let mut total_fail_requests: u64= 0;

    let total_requests: u64 = summary.stats_summaries.iter().map(|stat| {
        println!("task [{:4}] done: {} ms: {}", stat.connection_id, stat.requests_succeded, stat.duration.as_millis());
        total_success_requests += stat.requests_succeded;
        total_fail_requests += stat.requests_failed;
        stat.requests_succeded + stat.requests_failed
    }).sum();

    println!("======================================================================");
    println!("Total: {total_requests} Success: {total_success_requests} Failed: {total_fail_requests} Elapsed: {} ms Rate: {} req/s", summary.total_duration_ms, total_success_requests * 1000 / summary.total_duration_ms);

}