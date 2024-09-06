use error_stack::{Result, ResultExt};

use crate::{
    etcd::{EtcdClient, KvClient},
    object_store::ObjectETag,
};

static INGESTED_KEY: &str = "ingestion/ingested";

#[derive(Debug)]
pub struct IngestionStateClientError;

#[derive(Clone)]
pub struct IngestionStateClient {
    client: KvClient,
}

impl IngestionStateClient {
    pub fn new(client: &EtcdClient) -> Self {
        let client = client.kv_client();
        Self { client }
    }

    pub async fn get_ingested(&mut self) -> Result<Option<ObjectETag>, IngestionStateClientError> {
        let response = self
            .client
            .get(INGESTED_KEY)
            .await
            .change_context(IngestionStateClientError)
            .attach_printable("failed to get latest ingested block")?;

        let Some(kv) = response.kvs().first() else {
            return Ok(None);
        };

        let etag = String::from_utf8(kv.value().to_vec())
            .change_context(IngestionStateClientError)
            .attach_printable("failed to decode etag")?;

        Ok(Some(ObjectETag(etag)))
    }

    pub async fn put_ingested(
        &mut self,
        etag: ObjectETag,
    ) -> Result<(), IngestionStateClientError> {
        let value = etag.0;
        self.client
            .put(INGESTED_KEY, value.as_bytes())
            .await
            .change_context(IngestionStateClientError)
            .attach_printable("failed to put latest ingested block")?;

        Ok(())
    }
}

impl error_stack::Context for IngestionStateClientError {}

impl std::fmt::Display for IngestionStateClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ingestion state client error")
    }
}
