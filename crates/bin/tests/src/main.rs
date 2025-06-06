use std::str::FromStr;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand, ValueHint};
use primitive_types::U256;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, value_hint(ValueHint::Url))]
    node_url: String,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run end 2 end tests", long_about = "Run end 2 end tests")]
    E2e {
        #[arg(long, short)]
        feature: String,
    },
    #[command(
        about = "Display your balances accross all nodes",
        long_about = "Display your balances accross all nodes. For each node, show the total available amount for each unit."
    )]
    Concu {
        #[arg(long, short)]
        feature: String,
        #[arg(long, short)]
        number: u32,
    },
    /// Melt existing tokens
    #[command(
        about = "Melt some tokens",
        long_about = "Melt some tokens. Send them to the node and receive the original asset back."
    )]
    Stress {
        #[arg(long, short)]
        feature: String,
        #[arg(long, short)]
        number: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let node_url = cli.node_url;
    let node_url = wallet::types::NodeUrl::from_str(&node_url)?;
    let db_path = dirs::data_dir()
        .map(|mut dp| {
            dp.push("test-wallet.sqlite3");
            dp
        })
        .ok_or(anyhow!("couldn't find `data_dir` on this computer"))?;
    println!(
        "Using database at {:?}\n",
        db_path
            .as_path()
            .to_str()
            .ok_or(anyhow!("invalid db path"))?
    );
    let mut db_conn = rusqlite::Connection::open(db_path)?;

    let tx = db_conn.transaction()?;
    let (mut _node_client, node_id) = wallet::register_node(&tx, node_url.clone()).await?;
    tx.commit()?;
    println!(
        "Successfully registered {} as node with id `{}`",
        &node_url, node_id
    );
    Ok(())
}

pub fn parse_asset_amount(amount: &str) -> Result<U256, std::io::Error> {
    if amount.starts_with("0x") || amount.starts_with("0X") {
        U256::from_str_radix(amount, 16)
    } else {
        U256::from_str_radix(amount, 10)
    }
    .map_err(std::io::Error::other)
}
