mod input_params;

use clap::Parser;
use http_load_gen_lib::connection_task::{run, RunSettings, Report, Statistics};
use http_load_gen_lib::notification::{NotificationTask};
use input_params::InputParams;
use tokio::sync::mpsc::UnboundedReceiver;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::vec;
use tokio::time::Duration;
use tabled::{Tabled, Table, settings::{Alignment, Modify, Width, object::Rows, object::Columns}};

fn display_f(f: &f64) -> String {
    format!("{f:.2}")
}

#[derive(Tabled)]
struct TableRow {
    #[tabled(rename = "HTTP code")]
    status_code: u16,
    #[tabled(rename = "Request count")]
    requests: u64,
    #[tabled(rename = "Min. latency (μs)", display_with = "display_f")]
    min_latency: f64,
    #[tabled(rename = "Max. latency (μs)", display_with = "display_f")]
    max_latency: f64,
    #[tabled(rename = "Mean latency (μs)", display_with = "display_f")]
    mean_latency: f64,
    #[tabled(rename = "Standard deviation (μs)", display_with = "display_f")]
    std_deviation: f64,
    #[tabled(rename = "P90(μs)", display_with = "display_f")]
    p90: f64,
    #[tabled(rename = "P99(μs)", display_with = "display_f")]
    p99: f64 
}

impl From<Statistics> for TableRow {

    fn from(item: Statistics) -> Self {
        TableRow {
            status_code: item.code,
            requests: item.requests,
            min_latency: item.min,
            max_latency: item.max,
            mean_latency: item.mean,
            std_deviation: item.sd,
            p90: item.p90,
            p99: item.p99
        }
    }
}


#[tokio::main]
async fn main() {

    let cli = InputParams::parse();
    let target_url = cli.target_url.clone();
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<NotificationTask>();

    let run_settings = RunSettings {
        requests: cli.requests,
        connections: cli.connections,
        target_url: cli.target_url
    };

    let bar = ProgressBar::new(cli.requests);
    println!("Executing against: {}", target_url);

    let notif = tokio::spawn(get_updates(bar.clone(), rx));
    let forced_update = tokio::spawn(force_bar_update_at_interval(bar, Duration::from_millis(50)));
    let report = run(run_settings, Some(tx)).await.expect("Execution failed");

    let _ = notif.await;
    forced_update.abort();

    let statistics: Vec<TableRow> = report.get_stats().into_iter().map(|s| TableRow::from(s)).collect();

    write_report(&report, target_url.to_string(), cli.connections);
    write_table(statistics);
    write_file(cli.output_file, &report);
    

}

fn write_file(output_file: Option<String>, report: &Report) {
    if let Some(file_name) = output_file {
        match report.save_to_file(file_name.clone()) {
            Ok(_) => println!("Report was successfully saved to `{file_name}`"),
            Err(err) => eprintln!("Error: {err}")
        }
    }
}

fn write_report(report: &Report, target_url: String, num_connections: u16) {
    let request_count = report.get_requests_count();
    let elapsed_time = report.get_elapsed().as_millis();
    let successful_requests = report.get_successful_requests();
    let failed_requests = report.get_failed_requests();
    println!("Sent {request_count} requests in {elapsed_time}ms to {target_url} from {num_connections} connections");
    println!("Performed {successful_requests} ({failed_requests} failed) requests")
}

fn write_table(rows: Vec<TableRow>) {
    let mut table = Table::new(rows);
    table.with(Modify::new(Rows::new(0..)).with(Width::wrap(10).keep_words()));
    table.with(Modify::new(Columns::new(1..)).with(Alignment::right()));
    let table_str = table.to_string();
    println!("{table_str}");
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