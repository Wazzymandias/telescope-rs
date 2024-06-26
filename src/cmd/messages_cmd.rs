use clap::Args;
use eyre::eyre;

use crate::cmd::cmd::{BaseRpcConfig};
use crate::proto::hub_service_client::HubServiceClient;
use crate::proto::SyncIds;

#[derive(Args, Debug)]
pub struct MessagesCommand {
    #[clap(flatten)]
    base: BaseRpcConfig,

    #[arg(long)]
    sync_id: Option<String>,
}

impl MessagesCommand {
    pub async fn execute(&self) -> eyre::Result<()> {
        let tonic_endpoint = self.base.load_endpoint()?;
        let mut client = HubServiceClient::connect(tonic_endpoint).await.unwrap();
        let prefix = crate::cmd::cmd::parse_prefix(&self.sync_id)?;
        let response = client
            .get_all_messages_by_sync_ids(SyncIds {
                sync_ids: vec![prefix],
            })
            .await
            .unwrap();

        let str_response = serde_json::to_string_pretty(&response.into_inner());
        if str_response.is_err() {
            return Err(eyre!("{:?}", str_response.err()));
        }
        println!("{}", str_response.unwrap());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use tokio::runtime::Runtime;
    //
    // use crate::cmd::cmd::BaseConfig;
    //
    // use super::*;

    #[test]
    fn test_messages_command() {
        // let rt = Runtime::new().unwrap();
        // let result = rt.block_on(async {
        //     let base = BaseConfig {
        //         http: false,
        //         https: true,
        //         port: 8080,
        //         endpoint: "localhost".to_string(),
        //     };
        //     let messages_command = MessagesCommand {
        //         base,
        //         sync_id: Some("test".to_string()),
        //     };
        //     messages_command.execute().await.unwrap();
        // });
        //
        // assert!(result);
    }
}
