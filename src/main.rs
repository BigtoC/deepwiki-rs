use crate::{config::Config, generator::workflow::launch};
use anyhow::Result;

mod cache;
mod cli;
mod config;
mod generator;
mod llm;
mod memory;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::default(); // TODO:从CLI中读取传入

    launch(&config).await
}
