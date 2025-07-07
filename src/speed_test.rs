use crate::config::FileSize;
use futures_util::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::f64::{INFINITY, NEG_INFINITY};
use std::time::{Duration, Instant};
pub struct SpeedTestState {
    pub total_bytes_downloaded: u64,
    pub min_speed_mbps: f64,
    pub max_speed_mbps: f64,
    pub chunk_speeds_mbps: Vec<f64>,
}

impl SpeedTestState {
    pub fn new() -> Self {
        Self {
            total_bytes_downloaded: 0,
            min_speed_mbps: INFINITY,
            max_speed_mbps: NEG_INFINITY,
            chunk_speeds_mbps: Vec::new(),
        }
    }
}

pub async fn run_speed_test(
    client: &Client,
    url: &str,
    duration: u64,
    state: &mut SpeedTestState,
) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let main_pb = ProgressBar::new(duration);

    main_pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}s {msg}",
        )?
        .progress_chars("#>-"),
    );

    main_pb.set_message("Running test...");
    while start_time.elapsed() < Duration::from_secs(duration) {
        let current_download_start = Instant::now();
        let mut downloaded_bytes_this_round: u64 = 0;
        let response = match client.get(url).send().await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("\nError fetching URL: {}. Retrying...", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        };

        if !response.status().is_success() {
            eprintln!(
                "\nServer returned an error: {}. Retrying...",
                response.status()
            );
            tokio::time::sleep(Duration::from_secs(1)).await;
            continue;
        }

        let content_length = response.content_length();

        let download_pb = if let Some(len) = content_length {
            ProgressBar::new(len)
        } else {
            ProgressBar::hidden()
        };

        download_pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.blue/cyan}] {bytes}/{total_bytes} ({eta})",)?.progress_chars("##-"),);
        download_pb.set_message("Downloading chunk...");

        let mut stream = response.bytes_stream();

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("\nError reading chunk: {}. Aborting current download.", e);
                    break;
                }
            };
            downloaded_bytes_this_round += chunk.len() as u64;
            download_pb.set_position(downloaded_bytes_this_round);
            if start_time.elapsed() >= Duration::from_secs(duration) {
                break;
            }
        }

        download_pb.finish_and_clear();

        let download_duration = current_download_start.elapsed();
        if download_duration > Duration::ZERO {
            let speed_bps =
                (downloaded_bytes_this_round as f64 * 8.0) / download_duration.as_secs_f64();
            let speed_mbps = speed_bps / 1_000_000.0;

            println!(
                "  Chunk downloaded: {:.2} MB in {:.2?} ({:.2} Mbps)",
                downloaded_bytes_this_round as f64 / (1024.0 * 1024.0),
                download_duration,
                speed_mbps
            );

            state.chunk_speeds_mbps.push(speed_mbps);

            if speed_mbps < state.min_speed_mbps {
                state.min_speed_mbps = speed_mbps;
            }

            if speed_mbps > state.max_speed_mbps {
                state.max_speed_mbps = speed_mbps;
            }
        }

        state.total_bytes_downloaded += downloaded_bytes_this_round;
        main_pb.set_position(start_time.elapsed().as_secs());
    }
    main_pb.finish_with_message("Test complete!");
    Ok(())
}

pub fn get_url_for_size(size: &FileSize) -> &str {
    match size {
        FileSize::Mb100 => "https://nbg1-speed.hetzner.com/100MB.bin",
        FileSize::Gb1 => "https://nbg1-speed.hetzner.com/1GB.bin",
        FileSize::Gb10 => "https://nbg1-speed.hetzner.com/10GB.bin",
    }
}
