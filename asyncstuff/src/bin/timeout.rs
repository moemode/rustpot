use std::{future::Future, time::Duration};
use trpl::{self, Either};


async fn timeout<F: Future>(f: F, max_time:Duration) -> Result<F::Output, Duration> {
    // pass f to race first so it gets a chance to complete even if max_time is a very short duration
    let result = trpl::race(f, trpl::sleep(max_time)).await;
    match result {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
} 

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished"
        };
        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    })
}