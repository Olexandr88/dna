use apibara_dna_common::{
    cli::{EtcdArgs, ObjectStoreArgs},
    ingestion::{ingestion_service_loop, IngestionArgs},
    server::{server_loop, ServerArgs},
};
use clap::Args;
use error_stack::{Result, ResultExt};
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::{
    cli::rpc::RpcArgs, error::BeaconChainError, ingestion::BeaconChainBlockIngestion,
    scanner::BeaconChainScannerFactory,
};

#[derive(Args, Debug)]
pub struct StartCommand {
    #[clap(flatten)]
    rpc: RpcArgs,
    #[clap(flatten)]
    object_store: ObjectStoreArgs,
    #[clap(flatten)]
    etcd: EtcdArgs,
    #[clap(flatten)]
    ingestion: IngestionArgs,
    #[clap(flatten)]
    server: ServerArgs,
}

impl StartCommand {
    pub async fn run(self, ct: CancellationToken) -> Result<(), BeaconChainError> {
        info!("Starting Beaconchain DNA server");
        let provider = self.rpc.to_beacon_api_provider()?;
        let object_store = self.object_store.into_object_store_client().await;
        let mut etcd_client = self
            .etcd
            .into_etcd_client()
            .await
            .change_context(BeaconChainError)?;

        let status_response = etcd_client
            .status()
            .await
            .change_context(BeaconChainError)?;

        info!(
            version = status_response.version(),
            "connected to etcd cluster"
        );

        let ingestion_handle = if self.ingestion.ingestion_enabled {
            let ingestion_options = self.ingestion.to_ingestion_options();
            let ingestion = BeaconChainBlockIngestion::new(provider);
            tokio::spawn(ingestion_service_loop(
                ingestion,
                etcd_client.clone(),
                object_store.clone(),
                ingestion_options,
                ct.clone(),
            ))
        } else {
            tokio::spawn({
                let ct = ct.clone();
                async move {
                    ct.cancelled().await;
                    Ok(())
                }
            })
        };

        let scanner_factory = BeaconChainScannerFactory::new();

        let server_handle = if self.server.server_enabled {
            let options = self
                .server
                .to_server_options()
                .change_context(BeaconChainError)?;
            tokio::spawn(server_loop(
                scanner_factory,
                etcd_client,
                object_store,
                options,
                ct,
            ))
        } else {
            tokio::spawn({
                let ct = ct.clone();
                async move {
                    ct.cancelled().await;
                    Ok(())
                }
            })
        };

        tokio::select! {
            ingestion = ingestion_handle => {
                ingestion.change_context(BeaconChainError)?.change_context(BeaconChainError)?;
            }
            server = server_handle => {
                server.change_context(BeaconChainError)?.change_context(BeaconChainError)?;
            }
        }

        Ok(())
    }
}
