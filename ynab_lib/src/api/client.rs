use crate::api::types::Budget;
use crate::api::types::Response;
use anyhow::Result as AnyhowResult;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::time::Duration;
pub struct Client {
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Client {
        Client { api_key: api_key }
    }

    pub async fn get_budgets(&self) -> AnyhowResult<Vec<Budget>> {
        let url = format!("https://api.youneedabudget.com/v1/budgets");
        let client = reqwest::Client::new();
        let json = client
            .get(url)
            .header("Authorization", &self.api_key)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            budgets: Vec<Budget>,
        }
        let b: Response<Inner> = serde_json::from_str(&json).unwrap();

        Ok(b.data.budgets)
    }

    pub async fn get_budget(&self, budget_id: String) -> AnyhowResult<Budget> {
        let url = format!("https://api.youneedabudget.com/v1/budgets/{budget_id}");
        let client = reqwest::Client::new();
        let json = client
            .get(url)
            .header("Authorization", &self.api_key)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            budget: Budget,
        }
        let b: Response<Inner> = serde_json::from_str(&json).unwrap();

        Ok(b.data.budget)
    }
}
