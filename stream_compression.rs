use async_compression::tokio::write::ZstdEncoder;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::fs::File;

pub async fn compress_and_save(data: &str, path: &str) {
    let file = File::create(path).await.unwrap();
    let mut encoder = ZstdEncoder::new(BufWriter::new(file));
    encoder.write_all(data.as_bytes()).await.unwrap();
    encoder.shutdown().await.unwrap();
}
