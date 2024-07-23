use crate::state::system::TopicState;
use crate::streaming::storage::TopicStorage;
use crate::streaming::topics::topic::Topic;
use iggy::error::IggyError;

impl Topic {
    pub async fn load(&mut self, state: TopicState) -> Result<(), IggyError> {
        let storage = self.storage.clone();
        storage.topic.load(self, state).await?;
        Ok(())
    }

    pub async fn persist(&self) -> Result<(), IggyError> {
        self.storage.topic.save(self).await
    }

    pub async fn delete(&self, remove_from_disk: bool) -> Result<(), IggyError> {
        for partition in self.get_partitions() {
            partition.delete(remove_from_disk).await?;
        }

        if remove_from_disk {
            return self.storage.topic.delete(self).await;
        }
        Ok(())
    }

    pub async fn persist_messages(&self) -> Result<usize, IggyError> {
        let mut saved_messages_number = 0;
        for mut partition in self.get_partitions() {
            for segment in partition.get_segments_mut() {
                saved_messages_number += segment.persist_messages().await?;
            }
        }

        Ok(saved_messages_number)
    }

    pub async fn purge(&self, purge_storage: bool) -> Result<(), IggyError> {
        for mut partition in self.get_partitions() {
            partition.purge(purge_storage).await?;
        }
        Ok(())
    }
}
