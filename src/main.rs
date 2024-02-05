#![allow(unused_variables)]
#![allow(dead_code)]

use tokio;

mod bench_1;
mod bench_2;
mod bench_4;

#[tokio::main]
async fn main() {

    // let b1s = bench_1::benchmark();
    // println!("> 1-1million printer: {}", b1s);
    // let b2s = bench_2::benchmark().await;
    // println!("> 50-thread-jackhales: {}", b2s);

    let b4s = bench_4::benchmark();

}
