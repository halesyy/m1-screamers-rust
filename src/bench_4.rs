
/*
 * Do 1,000,000 sha256 hashes of random data.
 */

use rand;
use sha2::{Sha256, Digest};
use std::time::Instant;

pub fn benchmark() -> f32 {
   let start = Instant::now();
   for _ in 0..1_000_000 {
      do_hash();
   }
   let elapsed_ms = start.elapsed().as_millis() as f32;
   let elapsed_s = elapsed_ms / 1_000f32;
   println!("Completed 1m hashes in {}s", elapsed_s);
   elapsed_s
}

fn random_seed() -> String {
   let mut sd = String::new();
   for _ in 0..10 {
      sd.push(rand::random::<u8>() as char);
   }
   sd
}

fn do_hash() {
   let s = random_seed();   
   let mut hasher = Sha256::new();
   hasher.update(s);
   let result = hasher.finalize(); // A list of u8's.
}