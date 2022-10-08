use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub account_type: Option<String>,
    pub on_budget: Option<bool>,
    pub closed: Option<bool>,
    pub note: Option<String>,
    pub balance: Option<i64>,
    pub cleared_balance: Option<i64>,
    pub uncleared_balance: Option<i64>,
    pub transfer_payee_id: Option<String>,
    pub direct_import_linked: Option<bool>,
    pub direct_import_in_error: Option<bool>,
    pub deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Option<String>,
    pub date: Option<String>,
    pub amount: Option<i64>,
    pub memo: Option<String>,
    pub cleared: Option<String>,
    pub approved: Option<bool>,
    pub flag_color: Option<String>,
    pub account_id: Option<String>,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub deleted: Option<bool>,
    pub account_name: Option<String>,
    pub payee_name: Option<String>,
    pub category_name: Option<String>,
    pub subtransactions: Option<Vec<SubTransaction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTransaction {
    pub id: Option<String>,
    pub scheduled_transaction_id: Option<String>,
    pub amount: Option<i64>,
    pub memo: Option<String>,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTransaction {
    pub id: Option<String>,
    pub date_first: Option<String>,
    pub date_next: Option<String>,
    pub frequency: Option<RecurFrequency>,
    pub amount: Option<i64>,
    pub memo: Option<String>,
    pub flag_color: Option<String>,
    pub account_id: Option<String>,
    pub payee_id: Option<String>,
    pub category_id: Option<String>,
    pub transfer_account_id: Option<String>,
    pub deleted: Option<bool>,
    pub account_name: Option<String>,
    pub payee_name: Option<String>,
    pub category_name: Option<String>,
    pub subtransactions: Option<Vec<SubTransaction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Month {
    pub month: Option<String>,
    pub note: Option<String>,
    pub income: Option<i64>,
    pub budgeted: Option<i64>,
    pub activity: Option<i64>,
    pub to_be_budgeted: Option<i64>,
    pub age_of_money: Option<i64>,
    pub deleted: Option<bool>,
    pub categories: Option<Vec<Category>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<String>,
    pub category_group_id: Option<String>,
    pub name: Option<String>,
    pub hidden: Option<bool>,
    pub original_category_group_id: Option<String>,
    pub note: Option<String>,
    pub budgeted: Option<i64>,
    pub activity: Option<i64>,
    pub balance: Option<i64>,
    pub goal_type: Option<GoalType>,
    pub goal_creation_month: Option<String>,
    pub goal_target: Option<i64>,
    pub goal_target_month: Option<String>,
    pub goal_percentage_complete: Option<i64>,
    pub deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalType {
    #[serde(rename = "TB")]
    TargetBalance,
    #[serde(rename = "TBD")]
    TargetBalanceByDate,
    #[serde(rename = "MF")]
    MonthlyFunding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryGroup {
    pub id: String,
    pub name: String,
    pub hidden: Option<bool>,
    pub deleted: bool,
    pub transfer_account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payee {
    pub id: String,
    pub name: String,
    pub transfer_account_id: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayeeLocation {
    pub id: String,
    pub payee_id: String,
    pub latitude: String,
    pub longitude: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrencyFormat {
    pub iso_code: String,
    pub example_format: String,
    pub decimal_digits: i64,
    pub decimal_separator: String,
    pub symbol_first: bool,
    pub group_separator: String,
    pub currency_symbol: String,
    pub display_symbol: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DateFormat {
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: Option<String>,
    pub name: Option<String>,
    pub last_modified_on: Option<String>,
    pub first_month: Option<String>,
    pub last_month: Option<String>,
    pub date_format: Option<DateFormat>,
    pub currency_format: Option<CurrencyFormat>,
    pub accounts: Option<Vec<Account>>,
    pub payees: Option<Vec<CategoryGroup>>,
    pub payee_locations: Option<Vec<PayeeLocation>>,
    pub category_groups: Option<Vec<CategoryGroup>>,
    pub categories: Option<Vec<Category>>,
    pub months: Option<Vec<Month>>,
    pub transactions: Option<Vec<Transaction>>,
    pub subtransactions: Option<Vec<SubTransaction>>,
    pub scheduled_transactions: Option<Vec<ScheduledTransaction>>,
    pub scheduled_subtransactions: Option<Vec<SubTransaction>>,
    pub server_knowledge: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecurFrequency {
    Never,
    Daily,
    Weekly,
    EveryOtherWeek,
    TwiceAMonth,
    Every4Weeks,
    Monthly,
    EveryOtherMonth,
    Every3Months,
    Every4Months,
    TwiceAYear,
    Yearly,
    EveryOtherYear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub date_format: Option<DateFormat>,
    pub currency_format: Option<CurrencyFormat>,
}
