/*
 * Bench 2 is a threaded scraper. We open 50 threads,
 * and pull data from 50 websites of `jackhales.com`.
 */

#![allow(unused_imports)]
#![allow(dead_code)]

use reqwest;
use core::future::Future;
use futures::stream::{self, StreamExt};
use std::time::Instant;

const GET_URL: &str = "http://jackhales.com";

async fn get_page() -> Result<String, reqwest::Error> {
   let body = reqwest::get(GET_URL)
      .await?
      .text()
      .await?;
   Ok(body)
}

pub async fn benchmark() -> f32 {
   let start = Instant::now();
   let mut tasks = Vec::new();

   for _ in 0..50 {
      tasks.push(get_page());
   }

   let results = stream::iter(tasks)
      .buffer_unordered(50)
      .collect::<Vec<_>>()
      .await;

   let elapsed_ms = start.elapsed().as_millis() as f32;
   let elapsed_s = elapsed_ms / 1_000f32;

   println!("> Finished 50 GETs in {}s", elapsed_s);
   elapsed_s
}
