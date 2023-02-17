use std::{time::{Instant, Duration}, str::FromStr};
use hyper::{Client, client::HttpConnector, http::Uri, Body, StatusCode};
use anyhow::{Result, Context};

type HTTPClient = Client<HttpConnector, Body>;

struct TaskProps {
    num_of_requests: u64,
    uri: Uri
}

struct ConnectionStats {
    requests_executed: u64,
    requests_succeded: u64,
    requests_failed: u64,
    duration: Duration
}

impl ConnectionStats {
    fn new() -> Self {
        ConnectionStats {
            requests_executed: 0,
            requests_succeded: 0,
            requests_failed: 0,
            duration: Duration::ZERO
        }
    }

    fn add(&mut self, stats: ConnectionStats) {
        self.requests_executed += stats.requests_executed;
        self.requests_succeded += stats.requests_succeded;
        self.requests_failed += stats.requests_failed;
        self.duration += stats.duration;
    }
}

async fn task(connector: HTTPClient, task_props: TaskProps) -> Result<ConnectionStats, anyhow::Error> {
    let mut connection_stats = ConnectionStats::new();
    
    for _ in 0..task_props.num_of_requests {
        let now = Instant::now();
        let response = connector.get(task_props.uri.clone()).await?;
        let duration = now.elapsed();
        connection_stats.requests_executed += 1;
        connection_stats.duration += duration;

        match response.status() {
            StatusCode::OK => connection_stats.requests_succeded += 1,
            _ => connection_stats.requests_failed += 1
        }
    }

    Ok(connection_stats)
}

#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        dbg!(args);
        panic!("Usage: http-load-gen [URL]");
    }

    let url = &args[0];
    let uri = Uri::from_str(&url).unwrap();

    let num_of_tasks:u64 = 50_000;
    let num_of_connections: u64 = 4;
    let mut join_handles = Vec::new();

    let now = Instant::now();
    for _ in 0..num_of_connections {

        let client = Client::builder().build_http::<Body>();

        let task_props = TaskProps {
            num_of_requests: num_of_tasks / num_of_connections,
            uri: uri.clone()
        };

        join_handles.push(tokio::spawn(task(client, task_props)));
    }

    let mut final_connection_stats = ConnectionStats::new();

    for handle in join_handles {
        match handle.await.with_context(|| format!("Failed to execute")).expect("Task: ") {
            Ok(stats) => {
                println!("Total: {} Success: {} Failed: {} Elapsed: {} ms Rate: {} req/s", stats.requests_executed, stats.requests_succeded, stats.requests_failed, stats.duration.as_millis(), stats.requests_executed * 1000 / stats.duration.as_millis() as u64);
                final_connection_stats.add(stats)
            },
            Err(_) => println!("task failed")
        }
    }

    // let average = match now.elapsed().as_millis() as u64 {
    //     0 => 0,
    //     _ => num_of_tasks * 1000 / now.elapsed().as_millis() as u64
    // };
    let completion = now.elapsed().as_millis() as u64;
    println!("======================================================================");
    println!("Total: {} Success: {} Failed: {} Elapsed: {} ms Rate: {} req/s", final_connection_stats.requests_executed, final_connection_stats.requests_succeded, final_connection_stats.requests_failed, completion, final_connection_stats.requests_executed * 1000 / completion);

}
