use std::process::Stdio;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_health_endpoint_via_cortex_binary() {
    let url = std::env::var("CORTEX_URL").unwrap_or_else(|_| "http://127.0.0.1:8003".to_string());
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_cortex"))
        .env("CORTEX_DEV_MODE", "1")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to start cortex binary");

    let client = reqwest::Client::new();
    let health_url = format!("{url}/health");
    let mut healthy = false;

    for _ in 0..30 {
        match client.get(&health_url).send().await {
            Ok(response) if response.status().is_success() => {
                let body = response.text().await.expect("health body");
                assert!(body.contains("\"status\":\"ok\""));
                healthy = true;
                break;
            }
            _ => tokio::time::sleep(Duration::from_millis(500)).await,
        }
    }

    let _ = child.kill();
    let _ = child.wait();

    assert!(healthy, "cortex did not expose a healthy /health endpoint");
}
