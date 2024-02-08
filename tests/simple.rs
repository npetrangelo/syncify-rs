use std::time;
use async_std::task;
use syncify::syncify;

#[syncify]
fn one_second() {
    task::sleep(time::Duration::from_secs(1)).await;
}

fn sync_one_second() {
    futures::executor::block_on(task::sleep(time::Duration::from_secs(1)));
}

fn main() {}
