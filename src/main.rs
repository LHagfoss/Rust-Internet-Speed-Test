use clap::Parser;
use reqwest::Client;
use std::time::Instant;

mod about;
mod cli;
mod config;
mod logger;
mod results;
mod speed_test;

use cli::{Cli, Commands};
use speed_test::{SpeedTestState, run_speed_test};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Test { size, duration } => {
            let url = speed_test::get_url_for_size(size);

            println!("Starting internet speed test...");
            println!("Target URL: {}", url);
            println!("Test Duration: {} seconds", duration);
            println!("Test File Size: {:?}", size);

            let client = Client::new();
            let start_time = Instant::now();
            let mut state = SpeedTestState::new();

            if let Err(e) = run_speed_test(&client, url, *duration, &mut state).await {
                eprintln!("An error occurred during the speed test: {}", e);
                return Err(e);
            }

            let final_elapsed_time = start_time.elapsed();
            let total_bits_downloaded = state.total_bytes_downloaded as f64 * 8.0;
            let average_speed_mbps =
                total_bits_downloaded / final_elapsed_time.as_secs_f64() / 1_000_000.0;

            results::display_results(
                state.total_bytes_downloaded,
                final_elapsed_time,
                average_speed_mbps,
                state.min_speed_mbps,
                state.max_speed_mbps,
            );

            let result = results::create_result_struct(
                url,
                *duration,
                size,
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
        }
        Commands::About => {
            about::print_about();
        }
        Commands::Version => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
