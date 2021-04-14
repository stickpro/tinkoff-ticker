use tinkoff_api::apis::{configuration::{Configuration}, portfolio_api::portfolio_get, user_api::user_accounts_get};
mod tinkoff_api;
mod draw;
#[macro_use]
extern crate serde;
use std::error::Error;
use std::env;
use draw::*;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut config= Configuration::new();
    
    let api_key = env::var("TCS_API_KEY")?;

    config.bearer_access_token = Some(api_key);

    let user = user_accounts_get(&config).await?;
    let broker_account_id = first(&user.payload.accounts).unwrap().broker_account_id.to_string();
    let portfolio = portfolio_get(&config, Some(&broker_account_id.to_string())).await?;

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    loop {
        match draw(&mut terminal, &portfolio.payload.positions) {
            Ok(_) => {}
            Err(e) => println!("{:?}", e),
        };
    }
}

fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}