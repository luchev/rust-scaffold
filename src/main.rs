#![feature(trait_upcasting)]
#![feature(trivial_bounds)]

#[tokio::main]
async fn main() {
    let _ = app::run().await;
}
