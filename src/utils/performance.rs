use std::time::Instant;

use opencv::core;

pub fn measure<F, R>(label: &str, f: F) -> R
where
  F: FnOnce() -> R,
{
  let start = Instant::now();
  let result = f();
  println!(
    "⏱️ {}: {:.2}s",
    label,
    start.elapsed().as_secs_f64()
  );
  result
}

pub fn processed(i: usize, points: Vec<core::Point>) {
  if (i + 1) % 100 == 0 {
    println!(
      "Processed {}/{} points",
      i + 1,
      points.len()
    );
  }
}
