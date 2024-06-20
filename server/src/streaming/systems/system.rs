use crate::configs::server::PersonalAccessTokenConfig;
use crate::configs::system::SystemConfig;
use crate::streaming::cache::memory_tracker::CacheMemoryTracker;
use crate::streaming::clients::client_manager::ClientManager;
use crate::streaming::diagnostics::metrics::Metrics;
use crate::streaming::persistence::persister::*;
use crate::streaming::session::Session;
use crate::streaming::storage::SystemStorage;
use crate::streaming::streams::stream::Stream;
use crate::streaming::users::permissioner::Permissioner;
use iggy::error::IggyError;
use iggy::utils::crypto::{Aes256GcmEncryptor, Encryptor};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::{create_dir, remove_dir_all};
use tokio::time::Instant;
use tracing::{info, trace};

use crate::compat;
use crate::state::metadata::{FileMetadata, Metadata};
use crate::state::states::SystemState;
use crate::streaming::users::user::User;
use iggy::locking::IggySharedMut;
use iggy::locking::IggySharedMutFn;
use iggy::models::user_info::UserId;
use keepcalm::{SharedMut, SharedReadLock, SharedWriteLock};

#[derive(Debug)]
pub struct SharedSystem {
    system: SharedMut<System>,
}

impl SharedSystem {
    pub fn new(system: System) -> SharedSystem {
        SharedSystem {
            system: SharedMut::new(system),
        }
    }

    pub fn read(&self) -> SharedReadLock<System> {
        self.system.read()
    }

    pub fn write(&self) -> SharedWriteLock<System> {
        self.system.write()
    }
}

impl Clone for SharedSystem {
    fn clone(&self) -> Self {
        SharedSystem {
            system: self.system.clone(),
        }
    }
}

#[derive(Debug)]
pub struct System {
    pub permissioner: Permissioner,
    pub(crate) storage: Arc<SystemStorage>,
    pub(crate) streams: HashMap<u32, Stream>,
    pub(crate) streams_ids: HashMap<String, u32>,
    pub(crate) users: HashMap<UserId, User>,
    pub(crate) config: Arc<SystemConfig>,
    pub(crate) client_manager: IggySharedMut<ClientManager>,
    pub(crate) encryptor: Option<Box<dyn Encryptor>>,
    pub(crate) metrics: Metrics,
    pub(crate) metadata: Arc<dyn Metadata>,
    pub personal_access_token: PersonalAccessTokenConfig,
}

/// For each cache eviction, we want to remove more than the size we need.
/// This is done on purpose to avoid evicting messages on every write.
const CACHE_OVER_EVICTION_FACTOR: u64 = 5;

impl System {
    pub fn new(config: Arc<SystemConfig>, pat_config: PersonalAccessTokenConfig) -> System {
        let persister: Arc<dyn Persister> = match config.partition.enforce_fsync {
            true => Arc::new(FileWithSyncPersister {}),
            false => Arc::new(FilePersister {}),
        };
        let metadata = Arc::new(FileMetadata::new(
            &config.get_state_log_path(),
            persister.clone(),
        ));
        Self::create(
            config.clone(),
            SystemStorage::new(config, persister),
            metadata,
            pat_config,
        )
    }

    pub fn create(
        config: Arc<SystemConfig>,
        storage: SystemStorage,
        metadata: Arc<dyn Metadata>,
        pat_config: PersonalAccessTokenConfig,
    ) -> System {
        info!(
            "Server-side encryption is {}.",
            Self::map_toggle_str(config.encryption.enabled)
        );
        System {
            encryptor: match config.encryption.enabled {
                true => Some(Box::new(
                    Aes256GcmEncryptor::from_base64_key(&config.encryption.key).unwrap(),
                )),
                false => None,
            },
            config,
            streams: HashMap::new(),
            streams_ids: HashMap::new(),
            storage: Arc::new(storage),
            client_manager: IggySharedMut::new(ClientManager::default()),
            permissioner: Permissioner::default(),
            metrics: Metrics::init(),
            users: HashMap::new(),
            metadata,
            personal_access_token: pat_config,
        }
    }

    pub async fn init(&mut self) -> Result<(), IggyError> {
        let system_path = self.config.get_system_path();
        if !Path::new(&system_path).exists() && create_dir(&system_path).await.is_err() {
            return Err(IggyError::CannotCreateBaseDirectory(system_path));
        }

        let state_path = self.config.get_state_path();
        if !Path::new(&state_path).exists() && create_dir(&state_path).await.is_err() {
            return Err(IggyError::CannotCreateStateDirectory(state_path));
        }

        let streams_path = self.config.get_streams_path();
        if !Path::new(&streams_path).exists() && create_dir(&streams_path).await.is_err() {
            return Err(IggyError::CannotCreateStreamsDirectory(streams_path));
        }

        let runtime_path = self.config.get_runtime_path();
        if Path::new(&runtime_path).exists() && remove_dir_all(&runtime_path).await.is_err() {
            return Err(IggyError::CannotRemoveRuntimeDirectory(runtime_path));
        }

        if create_dir(&runtime_path).await.is_err() {
            return Err(IggyError::CannotCreateRuntimeDirectory(runtime_path));
        }

        info!(
            "Initializing system, data will be stored at: {}",
            self.config.get_system_path()
        );

        compat::storage_conversion::init(
            self.config.clone(),
            self.metadata.clone(),
            self.storage.clone(),
        )
        .await?;
        let entries = self.metadata.init().await?;
        let state = SystemState::init(entries).await?;
        info!("{state}");
        let now = Instant::now();
        self.load_version().await?;
        self.load_users(state.users.into_values().collect()).await?;
        self.load_streams(state.streams.into_values().collect())
            .await?;
        info!("Initialized system in {} ms.", now.elapsed().as_millis());
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), IggyError> {
        self.persist_messages().await?;
        Ok(())
    }

    pub async fn persist_messages(&self) -> Result<usize, IggyError> {
        trace!("Saving buffered messages on disk...");
        let mut saved_messages_number = 0;
        for stream in self.streams.values() {
            saved_messages_number += stream.persist_messages().await?;
        }

        Ok(saved_messages_number)
    }

    pub fn ensure_authenticated(&self, session: &Session) -> Result<(), IggyError> {
        match session.is_authenticated() {
            true => Ok(()),
            false => Err(IggyError::Unauthenticated),
        }
    }

    fn map_toggle_str<'a>(enabled: bool) -> &'a str {
        match enabled {
            true => "enabled",
            false => "disabled",
        }
    }

    pub async fn clean_cache(&self, size_to_clean: u64) {
        for stream in self.streams.values() {
            for topic in stream.get_topics() {
                for partition in topic.get_partitions().into_iter() {
                    tokio::task::spawn(async move {
                        let memory_tracker = CacheMemoryTracker::get_instance().unwrap();
                        let mut partition_guard = partition.write().await;
                        let cache = &mut partition_guard.cache.as_mut().unwrap();
                        let size_to_remove = (cache.current_size() as f64
                            / memory_tracker.usage_bytes() as f64
                            * size_to_clean as f64)
                            .ceil() as u64;
                        cache.evict_by_size(size_to_remove * CACHE_OVER_EVICTION_FACTOR);
                    });
                }
            }
        }
    }
}
