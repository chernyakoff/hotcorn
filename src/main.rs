mod app;
mod config;
use anyhow::Result;

use config::{Config, load_config};


#[tokio::main]
async fn main() -> Result<()> {
    let cfg: Config = load_config()?;
    //println!("Loaded config: {:?}", cfg);
    app::App::run(cfg).await?;
    Ok(())
}
