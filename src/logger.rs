use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTestResult {
    pub timestamp: String,
    pub url: String,
    pub test_duration_seconds: u64,
    pub test_file_size: String,
    pub total_data_downloaded_mb: f64,
    pub total_time_elapsed_seconds: f64,
    pub average_download_speed_mbps: f64,
    pub min_download_speed_mbps: Option<f64>,
    pub max_download_speed_mbps: Option<f64>,
    pub chunk_speeds_mbps: Vec<f64>,
}

pub async fn save_result(result: &SpeedTestResult) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string_pretty(result)?;
    let mut file = File::create("speed_test_results.json").await?;
    file.write_all(json_string.as_bytes()).await?;
    Ok(())
}
