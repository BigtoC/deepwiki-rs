use anyhow::Result;
use rig::{client::ProviderClient, providers::mistral::Client};

pub fn create_llm_client() -> Result<Client> {
    // let client = Client::builder("fs2wzco3o7haz38df1jo4vavnvauxtuz3f0b")
    //     .base_url("https://wanqing-api.corp.kuaishou.com/api/agent/v1/apps")
    //     .build()?;
    let client = Client::from_env();
    Ok(client)
}
