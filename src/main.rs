mod cli;

use clap::ArgMatches;
use cli::{
    constant::{
        SUB_COMMAND_ADD_TRANSACTION, SUB_COMMAND_GET_ACCOUNTS, SUB_COMMAND_GET_BUDGETS,
        SUB_COMMAND_GET_CATEGORIES, SUB_COMMAND_GET_PAYEES,
    },
    get_program,
    stdout::serialize_and_write,
    GetArgs,
};

use ynab_api_client::{
    failure::{Failure, Result},
    resource_access::ynab_ra,
    types::transaction::{Transaction, TransactionRequest},
};

#[tokio::main]
async fn main() -> Result<()> {
    let program = get_program();
    match program.subcommand() {
        (name, Some(matches)) if name == SUB_COMMAND_ADD_TRANSACTION => {
            do_add_transaction(matches).await
        }
        (name, Some(matches)) if name == SUB_COMMAND_GET_BUDGETS => do_get_budgets(matches).await,
        (name, Some(matches)) if name == SUB_COMMAND_GET_ACCOUNTS => do_get_accounts(matches).await,
        (name, Some(matches)) if name == SUB_COMMAND_GET_CATEGORIES => {
            do_get_categories(matches).await
        }
        (name, Some(matches)) if name == SUB_COMMAND_GET_PAYEES => do_get_payees(matches).await,
        _ => panic!("{}", program.usage()),
    }
}

async fn do_add_transaction(matches: &ArgMatches<'_>) -> Result<()> {
    let transaction = Transaction::from(matches.transaction());
    let request = TransactionRequest { transaction };

    let budget_id = matches.budget_id();
    let token = matches.token();

    let status = ynab_ra::add_transaction(&request, &budget_id, token).await?;
    match status.as_u16() {
        200..=299 => Ok(()),
        code => Err(Failure {
            reason: format!("Status code: {}", code),
        }),
    }
}

async fn do_get_budgets(matches: &ArgMatches<'_>) -> Result<()> {
    let token = matches.token();
    let budgets = ynab_ra::budgets(token).await?;
    serialize_and_write(&budgets)
}

async fn do_get_accounts(matches: &ArgMatches<'_>) -> Result<()> {
    let budget_id = matches.budget_id();
    let token = matches.token();

    let accounts = ynab_ra::accounts(budget_id, token).await?;
    serialize_and_write(&accounts)
}

async fn do_get_categories(matches: &ArgMatches<'_>) -> Result<()> {
    let budget_id = matches.budget_id();
    let token = matches.token();

    let categories = ynab_ra::categories(budget_id, token).await?;
    serialize_and_write(&categories)
}

async fn do_get_payees(matches: &ArgMatches<'_>) -> Result<()> {
    let budget_id = matches.budget_id();
    let token = matches.token();

    let payees = ynab_ra::payees(budget_id, token).await?;
    serialize_and_write(&payees)
}
