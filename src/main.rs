mod app;
mod cache;
mod common;
mod constants;
mod context;
mod datasources;
mod entities;
mod enums;
//mod keymaps;
mod presentation;
mod route;
mod states;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = datasources::configuration::get_configuration()?;

    app::main(config).await;
    Ok(())
}
