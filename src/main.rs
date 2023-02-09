use std::{time::{Instant}, str::FromStr};
use hyper::{Client, client::HttpConnector, http::Uri, Body};
use anyhow::{Result, Context};

type HTTPClient = Client<HttpConnector, Body>;
const URL: &str = "http://127.0.0.1:8080/person";

async fn task(client: HTTPClient, num_of_tasks: u64) -> Result<(), anyhow::Error> {
    for x in 0..num_of_tasks {
        let url = format!("{}?client_id={:p}&request_num={}", URL, &client, x);
        client.get(Uri::from_str(&url).unwrap()).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {

    let num_of_tasks:u64 = 50_000;
    let num_of_connections: u64 = 4;
    let mut join_handles = Vec::new();

    let now = Instant::now();

    for _ in 0..num_of_connections {
        let client = Client::builder().build_http::<Body>();
        let partial = num_of_tasks / num_of_connections;

        join_handles.push(tokio::spawn(task(client, partial)));
    }

    for handle in join_handles {
        let _ = handle.await.with_context(|| format!("Failed to execute")).expect("Task: ");
    }

    let average = match now.elapsed().as_millis() as u64 {
        0 => 0,
        _ => num_of_tasks * 1000 / now.elapsed().as_millis() as u64
    };
    println!("Requests: {}, Total duration: {} ms, Average: {} req/s", num_of_tasks, now.elapsed().as_millis(), average);

}
