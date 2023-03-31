use std::{str::FromStr};
use hyper::{http::Uri};
use http_load_gen::connection_task::{run, RunSettings};

#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        panic!("Usage: http-load-gen [URL]");
    }

    let url = &args[0];
    let uri = Uri::from_str(&url).expect(&format!("The provided URL \"{}\" is invalid", url));
    let run_settings = RunSettings::new(4, 1_000, uri);
    let summary = run(run_settings).await.expect("Execution failed");

    let total_requests = summary.stats_summaries.iter().fold(0, |accum, current| accum + current.requests_executed);
    let total_success_requests = summary.stats_summaries.iter().fold(0, |accum, current| accum + current.requests_succeded);
    let total_fail_requests = summary.stats_summaries.iter().fold(0, |accum, current| accum + current.requests_failed);

    println!("======================================================================");
    println!("Total: {total_requests} Success: {total_success_requests} Failed: {total_fail_requests} Elapsed: {} ms Rate: {} req/s", summary.total_duration_ms, total_success_requests * 1000 / summary.total_duration_ms);

}





#[cfg(test)]
mod tests {
    
    
    
}