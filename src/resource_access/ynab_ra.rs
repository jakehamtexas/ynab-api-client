use crate::failure::Result;
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

pub async fn accounts(budget_id: &str, token: &str) -> Result<Accounts> {
    let endpoint = get_endpoint("/budgets/{budget_id}/accounts", Some(budget_id));
    let response = get::<AccountsResponse>(&endpoint, token).await?;
    Ok(response.data)
}

pub async fn budgets(token: &str) -> Result<Budgets> {
    let endpoint = get_endpoint("/budgets", None);
    let response = get::<BudgetsResponse>(&endpoint, token).await?;
    Ok(response.data)
}

pub async fn categories(budget_id: &str, token: &str) -> Result<Categories> {
    let endpoint = get_endpoint("/budgets/{budget_id}/categories", Some(budget_id));
    let response = get::<CategoriesResponse>(&endpoint, token).await?;
    Ok(response.data)
}

pub async fn payees(budget_id: &str, token: &str) -> Result<Payees> {
    let endpoint = get_endpoint("/budgets/{budget_id}/payees", Some(budget_id));
    let response = get::<PayeesResponse>(&endpoint, token).await?;
    Ok(response.data)
}

pub async fn add_transaction(
    request: &TransactionRequest,
    budget_id: &str,
    token: &str,
) -> Result<StatusCode> {
    let endpoint = get_endpoint("/budgets/{budget_id}/transactions", Some(budget_id));
    let response = post(request, &endpoint, token).await?;
    Ok(response.status())
}

async fn get<U: DeserializeOwned>(endpoint: &str, token: &str) -> Result<U> {
    let client = client();
    let url = get_url(endpoint);
    let u = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<U>()
        .await?;
    Ok(u)
}

async fn post<T: Serialize>(body: &T, endpoint: &str, token: &str) -> Result<reqwest::Response> {
    let client = client();
    let url = get_url(endpoint);
    let response = client
        .post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    Ok(response)
}

fn get_endpoint(endpoint_base: &str, budget_id: Option<&str>) -> String {
    endpoint_base.replace("{budget_id}", budget_id.unwrap_or(""))
}

fn get_url(endpoint: &str) -> Url {
    Url::parse("https://api.youneedabudget.com")
        .ok()
        .and_then(|api_url_base| api_url_base.join(&format!("/v1{}", endpoint)).ok())
        .expect("URL parse error.")
}
