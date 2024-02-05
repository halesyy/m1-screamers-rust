
/*
 * This is meant to do threaded compute, where it
 * will do the following on multiple processes/threads:
 * - Store first 10m ints in a list
 * - Divide all by 5.2125
 * - Return the length of the list
 */

use std::time::Instant;

pub fn benchmark() -> f32 {
   let start = Instant::now(); 
   for _ in 0..50 {
      do_compute();
   }
   let elapsed_s = start.elapsed().as_millis() as f32;
   let elapsed_ms = elapsed_s / 1_000f32;
   println!("Finished 50 computations in {}s", elapsed_ms);
   elapsed_ms
}

fn do_compute() -> usize {
   let mut ints: Vec<f64> = Vec::new();
   
   for i in 0..10_000_000 {
      let val = (i as f64) / 5.2125;
      ints.push(val);
   }

   ints.len()
}