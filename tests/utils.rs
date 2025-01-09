use tokio::time;

pub async fn sleep(secs: u64) {
    time::sleep(time::Duration::from_secs(secs)).await
}