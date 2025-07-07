use crate::logger::SpeedTestResult;
use std::f64::{INFINITY, NEG_INFINITY};

pub fn display_results(
    total_bytes_downloaded: u64,
    final_elapsed_time: std::time::Duration,
    average_speed_mbps: f64,
    min_speed_mbps: f64,
    max_speed_mbps: f64,
) {
    println!("\n--- Speed Test Results ---");
    println!(
        "Total data downloaded: {:.2} MB",
        total_bytes_downloaded as f64 / (1024.0 * 1024.0)
    );
    println!("Total time elapsed: {:.2?}", final_elapsed_time);
    println!("Average download speed: {:.2} Mbps", average_speed_mbps);
    if min_speed_mbps != INFINITY {
        println!("Minimum chunk speed: {:.2} Mbps", min_speed_mbps);
    }
    if max_speed_mbps != NEG_INFINITY {
        println!("Maximum chunk speed: {:.2} Mbps", max_speed_mbps);
    }
}

pub fn create_result_struct(
    url: &str,
    duration: u64,
    size: &crate::config::FileSize,
    total_bytes_downloaded: u64,
    final_elapsed_time: std::time::Duration,
    average_speed_mbps: f64,
    min_speed_mbps: f64,
    max_speed_mbps: f64,
    chunk_speeds_mbps: Vec<f64>,
) -> SpeedTestResult {
    SpeedTestResult {
        timestamp: chrono::Utc::now().to_rfc3339(),
        url: url.to_string(),
        test_duration_seconds: duration,
        test_file_size: format!("{:?}", size),
        total_data_downloaded_mb: total_bytes_downloaded as f64 / (1024.0 * 1024.0),
        total_time_elapsed_seconds: final_elapsed_time.as_secs_f64(),
        average_download_speed_mbps: average_speed_mbps,
        min_download_speed_mbps: if min_speed_mbps != INFINITY {
            Some(min_speed_mbps)
        } else {
            None
        },
        max_download_speed_mbps: if max_speed_mbps != NEG_INFINITY {
            Some(max_speed_mbps)
        } else {
            None
        },
        chunk_speeds_mbps,
    }
}
