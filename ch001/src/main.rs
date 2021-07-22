use std::io::Write;

#[tokio::main]
async fn main() {
  let url = std::env::args().nth(1).expect("Usage: ch001 <url>");
  let mut body = reqwest::get(url.to_string()).await.unwrap();
  let full_size = body.content_length().unwrap();
  let mut file = std::fs::File::create(
    &body
      .url()
      .path_segments()
      .and_then(|segment| segment.last())
      .and_then(|name| if name.is_empty() { None } else { Some(name) })
      .unwrap_or("tmp.bin"),
  )
  .unwrap();
  let mut total_written = 0;
  while let Some(chunk) = body.chunk().await.unwrap() {
    let size = file.write(&chunk).unwrap();
    total_written += size;
    std::thread::spawn(print_progress(full_size, total_written));
  }
}

fn print_progress(full_size: u64, total_written: usize) {
  let mut w = 80.0;
  let ratio = 1.0 / (full_size as f64 / total_written as f64);
  while w * ratio > 0f64 {
    print!("#");
    w -= 1.0;
    std::thread::sleep(std::time::Duration::from_millis(100));
  }
  println!();
}
