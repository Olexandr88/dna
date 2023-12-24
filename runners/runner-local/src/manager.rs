use std::io::ErrorKind;

use std::process::Stdio;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use tracing::{info, warn};

use apibara_runner_common::error::{RunnerError, RunnerReportExt, RunnerResult, RunnerResultExt};

use apibara_runner_common::runner::v1::Indexer;

use error_stack::report;
use tokio::process::Command;

use crate::server::IndexerInfo;
use crate::utils::{build_indexer_command, refresh_status};

pub struct IndexerManager {
    pub indexers: Arc<Mutex<HashMap<String, IndexerInfo>>>,
}

impl Default for IndexerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexerManager {
    pub fn new() -> Self {
        Self {
            indexers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_indexer(&self, indexer_id: String, indexer: Indexer) -> RunnerResult<()> {
        let command = build_indexer_command(&indexer_id, &indexer)?;

        let port = portpicker::pick_unused_port().ok_or(RunnerError::internal(
            "Can't pick a port for the status server",
        ))?;

        let status_server_address = format!("0.0.0.0:{port}");

        let status_server_args = &[
            "--status-server-address".to_string(),
            status_server_address.clone(),
        ];

        let status_server_address = format!("http://{status_server_address}");

        let cmd = format!(
            "{} {} {}",
            command.program,
            command.args.join(" "),
            status_server_args.join(" ")
        );
        info!("Starting indexer {} with command `{}`", indexer.name, cmd);

        let stdout = tempfile::Builder::new()
            .prefix("apibara-local-runner-")
            .suffix(&format!("-{}-stdout.log", &indexer.name))
            .tempfile()
            .map_err(|err| report!(err).internal("cannot get temporary file for stdout"))
            .and_then(|keep| {
                keep.keep()
                    .internal("failed to keep temporary file for stdout")
            })
            .map(|(file, path)| {
                info!(
                    "Writing indexer {} stdout to `{}`",
                    indexer.name,
                    &path.display()
                );

                Stdio::from(file)
            })
            .unwrap_or_else(|err| {
                warn!(err =? err, "failed to write indexer stdout in temporary file");
                Stdio::piped()
            });

        let stderr = tempfile::Builder::new()
            .prefix("apibara-local-runner-")
            .suffix(&format!("-{}-stderr.log", &indexer.name))
            .tempfile()
            .map_err(|err| report!(err).internal("cannot get temporary file for stderr"))
            .and_then(|keep| {
                keep.keep()
                    .internal("failed to keep temporary file for stderr")
            })
            .map(|(file, path)| {
                info!(
                    "Writing indexer {} stderr to `{}`",
                    indexer.name,
                    &path.display()
                );

                Stdio::from(file)
            })
            .unwrap_or_else(|err| {
                warn!(err =? err, "failed to write indexer stderr in temporary file");
                Stdio::piped()
            });

        let child = Command::new(command.program)
            .current_dir(command.current_dir)
            .envs(command.envs)
            .args(command.args)
            .args(status_server_args)
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .map_err(|err| {
                let sink_type = &indexer.sink_type;

                if let ErrorKind::NotFound = err.kind() {
                    let error_message = format!(
                        "Sink {sink_type} is not installed\nInstall it with `apibara plugins install sink-{sink_type}` or by adding it to your $PATH",
                    );
                    RunnerError::not_found(sink_type).attach_printable(error_message)
                } else {
                    RunnerError::internal("failed to spawn indexer")
                }
            })?;

        let indexer_name = indexer.name.clone();

        let indexer_info = IndexerInfo {
            indexer_id,
            indexer,
            child,
            status_server_address,
        };

        self.indexers
            .lock()
            .await
            .insert(indexer_name, indexer_info);
        Ok(())
    }

    pub async fn refresh_status(&self, name: &str) -> RunnerResult<()> {
        let mut indexers = self.indexers.lock().await;

        let indexer_info = indexers.get_mut(name).ok_or(RunnerError::not_found(name))?;

        refresh_status(indexer_info).await?;

        Ok(())
    }

    pub async fn refresh_status_all(&self) -> RunnerResult<()> {
        let mut indexers = self.indexers.lock().await;

        let mut results = Vec::new();
        for indexer_info in indexers.values_mut() {
            results.push(refresh_status(indexer_info).await);
        }

        results.into_iter().collect()
    }

    pub async fn get_indexer(&self, name: &str) -> RunnerResult<Indexer> {
        let mut indexers = self.indexers.lock().await;

        let indexer_info = indexers.get_mut(name).ok_or(RunnerError::not_found(name))?;

        Ok(indexer_info.indexer.clone())
    }

    pub async fn delete_indexer(&self, name: &str) -> RunnerResult<()> {
        let mut indexers = self.indexers.lock().await;
        let indexer_info = indexers.get_mut(name).ok_or(RunnerError::not_found(name))?;

        let indexer_running = indexer_info
            .child
            .try_wait()
            .internal("failed to check status of indexer process")?
            .is_none();

        if indexer_running {
            indexer_info
                .child
                .kill()
                .await
                .internal("failed to kill process")?;
        }

        indexers.remove(name);

        Ok(())
    }

    pub async fn list_indexers(&self) -> RunnerResult<Vec<Indexer>> {
        let indexers = self.indexers.lock().await;

        Ok(indexers
            .values()
            .map(|indexer_info| indexer_info.indexer.clone())
            .collect())
    }

    pub async fn has_indexer(&self, name: &str) -> bool {
        let indexers = self.indexers.lock().await;
        indexers.contains_key(name)
    }
}
