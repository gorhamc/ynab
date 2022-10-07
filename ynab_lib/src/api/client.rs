use crate::api::types::*;
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
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            budgets: Vec<Budget>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.budgets)
    }

    pub async fn get_budget(&self, budget_id: String) -> AnyhowResult<Budget> {
        let url = format!("https://api.youneedabudget.com/v1/budgets/{budget_id}");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            budget: Budget,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.budget)
    }

    pub async fn get_budget_settings(&self, budget_id: String) -> AnyhowResult<Settings> {
        let url = format!("https://api.youneedabudget.com/v1/budgets/{budget_id}/settings");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            settings: Settings,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.settings)
    }

    pub async fn get_accounts(&self, budget_id: String) -> AnyhowResult<Vec<Account>> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/accounts");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            accounts: Vec<Account>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.accounts)
    }

    pub async fn get_account(
        &self,
        budget_id: String,
        account_id: String,
    ) -> AnyhowResult<Account> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/accounts/{account_id}");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            account: Account,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.account)
    }

    pub async fn get_categories(&self, budget_id: String) -> AnyhowResult<Vec<Category>> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/categories");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            categories: Vec<Category>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.categories)
    }

    pub async fn get_category(
        &self,
        budget_id: String,
        category_id: String,
    ) -> AnyhowResult<Category> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/categories/{category_id}");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            category: Category,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.category)
    }

    pub async fn get_category_by_month(
        &self,
        budget_id: String,
        category_id: String,
        month: String,
    ) -> AnyhowResult<Category> {
        let url = format!(
            "https://api.youneedabudget.com/v1/{budget_id}/months/{month}/categories/{category_id}"
        );
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            category: Category,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.category)
    }

    pub async fn get_payees(&self, budget_id: String) -> AnyhowResult<Vec<Payee>> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/payees");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            payees: Vec<Payee>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.payees)
    }

    pub async fn get_payee(&self, budget_id: String, payee_id: String) -> AnyhowResult<Payee> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/payees/{payee_id}");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            payee: Payee,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.payee)
    }

    pub async fn get_payee_locations(&self, budget_id: String) -> AnyhowResult<Vec<PayeeLocation>> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/payee_locations");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            payee_locations: Vec<PayeeLocation>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.payee_locations)
    }

    pub async fn get_payee_location(
        &self,
        budget_id: String,
        payee_location_id: String,
    ) -> AnyhowResult<PayeeLocation> {
        let url = format!(
            "https://api.youneedabudget.com/v1/{budget_id}/payee_locations/{payee_location_id}"
        );
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            payee_location: PayeeLocation,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.payee_location)
    }

    pub async fn get_locations_for_payee(
        &self,
        budget_id: String,
        payee_id: String,
    ) -> AnyhowResult<Vec<PayeeLocation>> {
        let url = format!(
            "https://api.youneedabudget.com/v1/{budget_id}/payees/{payee_id}/payee_locations"
        );
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            payee_locations: Vec<PayeeLocation>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.payee_locations)
    }

    pub async fn get_months(&self, budget_id: String) -> AnyhowResult<Vec<Month>> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/months");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            months: Vec<Month>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.months)
    }

    pub async fn get_month(&self, budget_id: String, month: String) -> AnyhowResult<Month> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/months/{month}");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            month: Month,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.month)
    }

    pub async fn get_transactions(&self, budget_id: String) -> AnyhowResult<Vec<Transaction>> {
        let url = format!("https://api.youneedabudget.com/v1/{budget_id}/transactions");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            transactions: Vec<Transaction>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.transactions)
    }

    pub async fn get_transaction(
        &self,
        budget_id: String,
        transaction_id: String,
    ) -> AnyhowResult<Transaction> {
        let url =
            format!("https://api.youneedabudget.com/v1/{budget_id}/transactions/{transaction_id}");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            transaction: Transaction,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.transaction)
    }

    pub async fn get_account_transactions(
        &self,
        budget_id: String,
        account_id: String,
    ) -> AnyhowResult<Vec<Transaction>> {
        let url = format!(
            "https://api.youneedabudget.com/v1/{budget_id}/accounts/{account_id}/transactions"
        );
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            transactions: Vec<Transaction>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.transactions)
    }

    pub async fn get_category_transactions(
        &self,
        budget_id: String,
        category_id: String,
    ) -> AnyhowResult<Vec<Transaction>> {
        let url = format!(
            "https://api.youneedabudget.com/v1/{budget_id}/categories/{category_id}/transactions"
        );
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            transactions: Vec<Transaction>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.transactions)
    }

    pub async fn get_payee_transactions(
        &self,
        budget_id: String,
        payee_id: String,
    ) -> AnyhowResult<Vec<Transaction>> {
        let url =
            format!("https://api.youneedabudget.com/v1/{budget_id}/payees/{payee_id}/transactions");
        let client = reqwest::Client::new();
        let token = format!("Bearer {}", &self.api_key);

        let json = client
            .get(url)
            .header("Authorization", token)
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            transactions: Vec<Transaction>,
        }
        let b: Response<Inner> = serde_json::from_str(&json)?;

        Ok(b.data.transactions)
    }
}
