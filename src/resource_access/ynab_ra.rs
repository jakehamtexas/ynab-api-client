use crate::types::{
    account::{Accounts, AccountsResponse},
    budget::{Budgets, BudgetsResponse},
    category::{Categories, CategoriesResponse},
    payee::{Payees, PayeesResponse},
    transaction::TransactionRequest,
};

use reqwest::{Client, ClientBuilder, StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};

fn client() -> Client {
    ClientBuilder::new().build().expect(
        "Cannot initialize TLS backend, or the resolver cannot load the system configuration.",
    )
}

pub async fn accounts(budget_id: &str, token: &str) -> reqwest::Result<Accounts> {
    let endpoint = get_endpoint("/budgets/{budget_id}/accounts", Some(budget_id));
    let response = get::<AccountsResponse>(&endpoint, token).await?;
    Ok(response.data)
}

pub async fn budgets(token: &str) -> reqwest::Result<Budgets> {
    let endpoint = get_endpoint("/budgets", None);
    let response = get::<BudgetsResponse>(&endpoint, token).await?;
    Ok(response.data)
}

pub async fn categories(budget_id: &str, token: &str) -> reqwest::Result<Categories> {
    let endpoint = get_endpoint("/budgets/{budget_id}/categories", Some(budget_id));
    let response = get::<CategoriesResponse>(&endpoint, token).await?;
    Ok(response.data)
}

pub async fn payees(budget_id: &str, token: &str) -> reqwest::Result<Payees> {
    let endpoint = get_endpoint("/budgets/{budget_id}/payees", Some(budget_id));
    let response = get::<PayeesResponse>(&endpoint, token).await?;
    Ok(response.data)
}

pub async fn add_transaction(
    request: &TransactionRequest,
    budget_id: &str,
    token: &str,
) -> reqwest::Result<StatusCode> {
    let endpoint = get_endpoint("/budgets/{budget_id}/transactions", Some(budget_id));
    let response = post(request, &endpoint, token).await?;
    Ok(response.status())
}

async fn get<U: DeserializeOwned>(endpoint: &str, token: &str) -> reqwest::Result<U> {
    let client = client();
    let url = get_url(endpoint);
    client
        .get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<U>()
        .await
}

async fn post<T: Serialize>(
    body: &T,
    endpoint: &str,
    token: &str,
) -> reqwest::Result<reqwest::Response> {
    let client = client();
    let url = get_url(endpoint);
    client.post(url).bearer_auth(token).json(&body).send().await
}

fn get_endpoint(endpoint_base: &str, budget_id: Option<&str>) -> String {
    endpoint_base.replace("{budget_id}", budget_id.unwrap_or(""))
}

fn get_url(endpoint: &str) -> Url {
    Url::parse("https://api.youneedabudget.com/v1")
        .ok()
        .and_then(|api_url_base| api_url_base.join(endpoint).ok())
        .expect("URL parse error.")
}
