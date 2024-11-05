use std::sync::Arc;

use alloy_rpc_types::BlockNumberOrTag;
use apibara_etcd::EtcdClient;
use error_stack::Result;
use foyer::HybridCacheBuilder;
use testcontainers::{runners::AsyncRunner, ContainerAsync};

use apibara_dna_common::{
    chain::BlockInfo,
    file_cache::FileCache,
    fragment,
    ingestion::{
        state_client::testing::{etcd_server_container, EtcdServer, EtcdServerExt},
        BlockIngestion, IngestionError, IngestionService, IngestionServiceOptions,
        IngestionStateClient,
    },
    object_store::{
        testing::{minio_container, MinIO, MinIOExt},
        ObjectStore, ObjectStoreOptions,
    },
    Cursor, Hash,
};
use testing::{
    anvil_server_container, AnvilProvider, AnvilProviderExt, AnvilServer, AnvilServerExt,
};

async fn init_minio() -> (ContainerAsync<MinIO>, ObjectStore) {
    let minio = minio_container().start().await.unwrap();
    let config = minio.s3_config().await;

    let client = ObjectStore::new_from_config(
        config,
        ObjectStoreOptions {
            bucket: "test".to_string(),
            ..Default::default()
        },
    );

    client.ensure_bucket().await.unwrap();

    (minio, client)
}

async fn init_etcd_server() -> (ContainerAsync<EtcdServer>, EtcdClient) {
    let etcd_server = etcd_server_container().start().await.unwrap();
    let etcd_client = etcd_server.etcd_client().await;

    (etcd_server, etcd_client)
}

async fn init_anvil() -> (ContainerAsync<AnvilServer>, Arc<AnvilProvider>) {
    let anvil_server = anvil_server_container().start().await.unwrap();

    let provider = anvil_server.alloy_provider().await;
    (anvil_server, provider)
}

async fn init_file_cache() -> FileCache {
    HybridCacheBuilder::default()
        .memory(1024 * 1024)
        .storage(foyer::Engine::Large)
        .build()
        .await
        .expect("failed to create file cache")
}

#[tokio::test]
async fn test_ingestion_initialize() {
    let (_minio, object_store) = init_minio().await;
    let (_etcd_server, etcd_client) = init_etcd_server().await;
    let (_anvil_server, anvil_provider) = init_anvil().await;

    let mut state_client = IngestionStateClient::new(&etcd_client);
    let file_cache = init_file_cache().await;

    let block_ingestion = TestBlockIngestion {
        provider: anvil_provider.clone(),
    };

    let mut service = IngestionService::new(
        block_ingestion,
        etcd_client,
        object_store,
        file_cache,
        IngestionServiceOptions::default(),
    );

    anvil_provider.anvil_mine(100, 3).await;
    let header = anvil_provider.get_header(BlockNumberOrTag::Latest).await;
    assert_eq!(header.number, 100);

    let starting_state = service.initialize().await.unwrap();
    let ingest_state = starting_state.as_ingest().unwrap();

    assert_eq!(ingest_state.head.number, 100);

    let starting_block = state_client.get_starting_block().await.unwrap();
    assert_eq!(starting_block, Some(0));

    let finalized = state_client.get_finalized().await.unwrap();
    assert!(finalized.is_some());
    assert_eq!(ingest_state.finalized.number, finalized.unwrap());

    // Ingested is updated only after a chain segment is uploaded.
    let ingested = state_client.get_ingested().await.unwrap();
    assert!(ingested.is_none());
}

#[tokio::test]
async fn test_ingestion_initialize_with_starting_block() {
    let (_minio, object_store) = init_minio().await;
    let (_etcd_server, etcd_client) = init_etcd_server().await;
    let (_anvil_server, anvil_provider) = init_anvil().await;

    let mut state_client = IngestionStateClient::new(&etcd_client);
    let file_cache = init_file_cache().await;

    let block_ingestion = TestBlockIngestion {
        provider: anvil_provider.clone(),
    };

    let options = IngestionServiceOptions {
        override_starting_block: Some(100),
        ..Default::default()
    };

    let mut service = IngestionService::new(
        block_ingestion,
        etcd_client,
        object_store,
        file_cache,
        options,
    );

    anvil_provider.anvil_mine(200, 3).await;
    let header = anvil_provider.get_header(BlockNumberOrTag::Latest).await;
    assert_eq!(header.number, 200);

    let starting_state = service.initialize().await.unwrap();
    let ingest_state = starting_state.as_ingest().unwrap();

    assert_eq!(ingest_state.head.number, 200);

    let starting_block = state_client.get_starting_block().await.unwrap();
    assert_eq!(starting_block, Some(100));

    let finalized = state_client.get_finalized().await.unwrap();
    assert!(finalized.is_some());
    assert_eq!(ingest_state.finalized.number, finalized.unwrap());

    // Ingested is updated only after a chain segment is uploaded.
    let ingested = state_client.get_ingested().await.unwrap();
    assert!(ingested.is_none());
}

#[derive(Clone)]
struct TestBlockIngestion {
    provider: Arc<AnvilProvider>,
}

impl BlockIngestion for TestBlockIngestion {
    async fn get_head_cursor(&self) -> Result<Cursor, IngestionError> {
        let header = self.provider.get_header(BlockNumberOrTag::Latest).await;
        let hash = Hash(header.hash.to_vec());
        Ok(Cursor::new(header.number, hash))
    }

    async fn get_finalized_cursor(&self) -> Result<Cursor, IngestionError> {
        let header = self.provider.get_header(BlockNumberOrTag::Finalized).await;
        let hash = Hash(header.hash.to_vec());
        Ok(Cursor::new(header.number, hash))
    }

    async fn get_block_info_by_number(&self, number: u64) -> Result<BlockInfo, IngestionError> {
        let header = self
            .provider
            .get_header(BlockNumberOrTag::Number(number))
            .await;
        let hash = Hash(header.hash.to_vec());
        let parent_hash = Hash(header.parent_hash.to_vec());

        Ok(BlockInfo {
            number,
            hash,
            parent: parent_hash,
        })
    }

    async fn ingest_block_by_number(
        &self,
        number: u64,
    ) -> Result<(BlockInfo, fragment::Block), IngestionError> {
        let info = self.get_block_info_by_number(number).await?;

        let header = fragment::HeaderFragment {
            data: Vec::default(),
        };
        let index = fragment::IndexGroupFragment {
            indexes: Vec::default(),
        };
        let join = fragment::JoinGroupFragment {
            joins: Vec::default(),
        };

        let block = fragment::Block {
            header,
            index,
            join,
            body: Vec::default(),
        };

        Ok((info, block))
    }
}

pub mod testing {
    use std::sync::Arc;

    use alloy_provider::{network::Ethereum, Provider, ProviderBuilder, RootProvider};
    use alloy_rpc_client::ClientBuilder;
    use alloy_rpc_types::{BlockId, BlockNumberOrTag, BlockTransactionsKind, Header};
    use alloy_transport_http::Http;
    use futures::Future;
    use reqwest::Client;
    use testcontainers::{core::ContainerPort, ContainerAsync, Image};
    use url::Url;

    pub struct AnvilServer;

    pub type AnvilProvider = RootProvider<Http<Client>, Ethereum>;

    pub trait AnvilServerExt {
        fn alloy_provider(&self) -> impl Future<Output = Arc<AnvilProvider>> + Send;
    }

    pub trait AnvilProviderExt {
        fn anvil_mine(
            &self,
            block_count: u64,
            interval_sec: u64,
        ) -> impl Future<Output = ()> + Send;

        fn get_header(&self, block: BlockNumberOrTag) -> impl Future<Output = Header> + Send;
    }

    impl Image for AnvilServer {
        fn name(&self) -> &str {
            "anvil"
        }

        fn tag(&self) -> &str {
            "latest"
        }

        fn expose_ports(&self) -> &[ContainerPort] {
            &[ContainerPort::Tcp(8545)]
        }

        fn ready_conditions(&self) -> Vec<testcontainers::core::WaitFor> {
            Vec::default()
        }
    }

    pub fn anvil_server_container() -> AnvilServer {
        AnvilServer
    }

    impl AnvilServerExt for ContainerAsync<AnvilServer> {
        async fn alloy_provider(&self) -> Arc<AnvilProvider> {
            let port = self
                .get_host_port_ipv4(8545)
                .await
                .expect("Anvil port 8545 not found");

            let url: Url = format!("http://localhost:{port}").parse().unwrap();
            let client = ClientBuilder::default().http(url);
            let provider = ProviderBuilder::default().on_client(client);

            Arc::new(provider)
        }
    }

    impl AnvilProviderExt for Arc<AnvilProvider> {
        async fn anvil_mine(&self, block_count: u64, interval_sec: u64) {
            self.raw_request::<_, serde_json::Value>(
                "anvil_mine".into(),
                &(block_count, interval_sec),
            )
            .await
            .expect("anvil_mine request failed");
        }

        async fn get_header(&self, block: BlockNumberOrTag) -> Header {
            let response = self
                .get_block(BlockId::Number(block), BlockTransactionsKind::Hashes)
                .await
                .expect("get_header request failed")
                .expect("get_header block missing");
            response.header
        }
    }
}
