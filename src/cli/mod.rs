use clap::{App, Arg, ArgMatches, SubCommand};
use constant::{
    ARG_NAME_TOKEN, ARG_NAME_TRANSACTION, SUB_COMMAND_ADD_TRANSACTION, SUB_COMMAND_GET_ACCOUNTS,
    SUB_COMMAND_GET_CATEGORIES, SUB_COMMAND_GET_PAYEES,
};

use self::constant::{ARG_NAME_BUDGET_ID, SUB_COMMAND_GET_BUDGETS};
pub mod constant;
pub mod stdout;
pub fn get_program<'a>() -> ArgMatches<'a> {
    let version = "1.0";
    let author = "Jake Hamilton <jakehamtexas@gmail.com";

    let token_arg = Arg::with_name(ARG_NAME_TOKEN)
        .short("t")
        .long(ARG_NAME_TOKEN)
        .help("The bearer token for the YNAB API.")
        .required(true)
        .takes_value(true);
    let budget_id_arg = Arg::with_name(ARG_NAME_BUDGET_ID)
        .short("b")
        .long(ARG_NAME_BUDGET_ID)
        .required(true)
        .help("The budget ID.")
        .takes_value(true);

    App::new("ynab-api-client")
        .version(version)
        .author(author)
        .about("A non-exhaustive CLI for using the YNAB API.")
        .subcommand(
            SubCommand::with_name(SUB_COMMAND_ADD_TRANSACTION)
                .about("Adds a transaction.")
                .version(version)
                .author(author)
                .arg(&token_arg)
                .arg(
                    Arg::with_name(ARG_NAME_TRANSACTION)
                        .short("x")
                        .long(ARG_NAME_TRANSACTION)
                        .help("The transaction JSON.")
                        .required(true)
                        .takes_value(true),
                )
                .arg(&budget_id_arg),
        )
        .subcommand(
            SubCommand::with_name(SUB_COMMAND_GET_BUDGETS)
                .about("Gets names and ids for the budgets, as well as the default budget if it exists.")
                .version(version)
                .author(author)
                .arg(&token_arg),
        )
        .subcommand(
            SubCommand::with_name(SUB_COMMAND_GET_ACCOUNTS)
                .about("Gets names and ids for the accounts on a specific budget.")
                .version(version)
                .author(author)
                .arg(&budget_id_arg)
                .arg(&token_arg),
        )
        .subcommand(
            SubCommand::with_name(SUB_COMMAND_GET_CATEGORIES)
                .about("Gets names and ids for the categories on a specific budget.")
                .version(version)
                .author(author)
                .arg(&budget_id_arg)
                .arg(&token_arg),
        )
        .subcommand(
            SubCommand::with_name(SUB_COMMAND_GET_PAYEES)
                .about("Gets names and ids for the payees on a specific budget.")
                .version(version)
                .author(author)
                .arg(&budget_id_arg)
                .arg(&token_arg),
        )
        .get_matches()
}

pub trait GetArgs {
    fn safe_value_of(&self, v: &str) -> &str;
    fn budget_id(&self) -> &str;
    fn token(&self) -> &str;
    fn transaction(&self) -> &str;
}

impl GetArgs for ArgMatches<'_> {
    fn safe_value_of(&self, v: &str) -> &str {
        self.value_of(v).unwrap()
    }

    fn budget_id(&self) -> &str {
        self.safe_value_of(ARG_NAME_BUDGET_ID)
    }

    fn token(&self) -> &str {
        self.safe_value_of(ARG_NAME_TOKEN)
    }

    fn transaction(&self) -> &str {
        self.safe_value_of(ARG_NAME_TRANSACTION)
    }
}
