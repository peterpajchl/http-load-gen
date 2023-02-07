use std::time::{Instant};
use hyper::{Client, client::HttpConnector, http::Uri, Body};
use anyhow::{Result, Context};

type HTTPClient = Client<HttpConnector, Body>;

async fn task(client: HTTPClient, num_of_tasks: u128) -> Result<(), anyhow::Error> {
    
    for _ in 0..num_of_tasks {
        let uri = Uri::from_static("http://127.0.0.1:8080/person");
        client.get(uri).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    
    let client: HTTPClient = Client::builder().build_http::<Body>();
    let num_of_tasks:u128 = 50;

    let _ = tokio::spawn(async move {
        let now = Instant::now();
        task(client, num_of_tasks).await
            .with_context(|| format!("Failed to fetch"))
            .expect("Task: ");
        let average = now.elapsed().as_micros() / num_of_tasks;
        println!("Requests: {}, Total duration: {} ms, Average duration: {} micro/pr", num_of_tasks, now.elapsed().as_millis(), average);
    }).await;

}
