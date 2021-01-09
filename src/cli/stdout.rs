use serde::Serialize;
use std::io::Write;
use ynab_api_client::failure::Result;

pub fn serialize_and_write<T: Serialize>(v: &T) -> Result<()> {
    let serialized = serde_json::to_string_pretty(v)?;
    print!("{}", serialized);
    std::io::stdout().flush()?;
    Ok(())
}
