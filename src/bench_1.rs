/*
 * This benchmark runs a print from
 * 1 - 1,000,000. It more tests the
 * terminal than the execution.
 */
use std::time::Instant;

pub fn benchmark() -> f32 {
   let now = Instant::now();
   for i in 0..1_000_000 {
      println!("{}", i);
   }

   // Parse into seconds as a float.
   let elapsed_ms = now.elapsed().as_millis() as f32;
   let elapsed_s = elapsed_ms / 1_000f32;

   println!("1: {}", elapsed_s);
   elapsed_s
}

