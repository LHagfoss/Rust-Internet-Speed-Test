use clap::Parser;
use reqwest::Client;
use std::time::Instant;

mod config;
mod logger;
mod results;
mod speed_test;

use config::Args;
use speed_test::{run_speed_test, SpeedTestState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = speed_test::get_url_for_size(&args.size);

    println!("Starting internet speed test...");
    println!("Target URL: {}", url);
    println!("Test Duration: {} seconds", args.duration);
    println!("Test File Size: {:?}", args.size);

    let client = Client::new();
    let start_time = Instant::now();
    let mut state = SpeedTestState::new();

    if let Err(e) = run_speed_test(&client, url, args.duration, &mut state).await {
        eprintln!("An error occurred during the speed test: {}", e);
        return Err(e);
    }

    let final_elapsed_time = start_time.elapsed();
    let total_bits_downloaded = state.total_bytes_downloaded as f64 * 8.0;
    let average_speed_mbps = total_bits_downloaded / final_elapsed_time.as_secs_f64() / 1_000_000.0;

    results::display_results(
        state.total_bytes_downloaded,
        final_elapsed_time,
        average_speed_mbps,
        state.min_speed_mbps,
        state.max_speed_mbps,
    );

    let result = results::create_result_struct(
        url,
        args.duration,
        &args.size,
        state.total_bytes_downloaded,
        final_elapsed_time,
        average_speed_mbps,
        state.min_speed_mbps,
        state.max_speed_mbps,
        state.chunk_speeds_mbps,
    );

    if let Err(e) = logger::save_result(&result).await {
        eprintln!("Error saving results to file: {}", e);
    } else {
        println!("Results saved to speed_test_results.json");
    }

    Ok(())
}
