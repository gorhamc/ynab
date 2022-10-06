use anyhow::Result;
use clap::Parser;
use dotenv;
use owo_colors::colors::*;
use owo_colors::OwoColorize;
//use reqwest;
//use serde_json::Value;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use ynab_lib::api::client::Client;
use ynab_lib::api::types::Budget;
use ynab_lib::api::types::Response;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    budget: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let api_key = dotenv::var("API_KEY").unwrap();
    let token = format!("Bearer {api_key}");

    //let budget_id = dotenv::var("BUDGET_ID");
    let budget_id = String::from("4911692b-d1d0-4ed5-aa43-f80e598d509f");

    let args = Args::parse();
    println!("Hello {}!", args.budget.fg::<Yellow>().bg::<Red>());

    let client = Client::new(api_key);
    let budgets = client.get_budgets();
    let budget = client.get_budget(budget_id);

    //print_a_budget(budget)?;
    Ok(())
}

fn print_a_budget(json: String) -> Result<()> {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Inner {
        budget: Budget,
    }
    let b: Response<Inner> = serde_json::from_str(&json)?;
    println!("{:?}", b);
    Ok(())
}