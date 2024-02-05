#![allow(unused_variables)]
#![allow(dead_code)]

use tokio;

mod bench_1;
mod bench_2;

#[tokio::main]
async fn main() {

    // let b1s = bench_1::benchmark();
    // println!("> 1-1million printer: {}", b1s);
    
    let res = bench_2::benchmark().await;
    
    

}
