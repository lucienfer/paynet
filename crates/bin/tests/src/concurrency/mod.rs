use std::str::FromStr;

use wallet::{connect_to_node, types::NodeUrl};

use crate::{concurrency::concurrence_ops::swap_concurrence, env_variables::EnvVariables};

mod concurrence_ops;

pub async fn run_concurrency(env: EnvVariables) -> anyhow::Result<()> {
    println!("[CONCURRENCY] Launching concurrency tests");
    let node_url = NodeUrl::from_str(&env.node_url)?;
    let node_client = connect_to_node(&node_url).await?;

    swap_concurrence(node_client, env).await?;

    println!("✅ [CONCURRENCY] All tasks completed successfully — concurrency test passed.\n");
    Ok(())
}
