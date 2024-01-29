use crate::{
    error::ContainerError,
    storage::{self, types},
};

#[async_trait::async_trait]
impl<T> storage::HashInterface for super::Caching<T, types::HashTable>
where
    T: storage::HashInterface
        + storage::Cacheable<types::HashTable, Key = Vec<u8>, Value = types::HashTable>
        + Sync
        + Send,
{
    type Error = T::Error;

    async fn find_by_data_hash(
        &self,
        data_hash: &[u8],
    ) -> Result<Option<types::HashTable>, ContainerError<Self::Error>> {
        match self.lookup(data_hash.to_vec()).await {
            value @ Some(_) => Ok(value),
            None => Ok(match self.inner.find_by_data_hash(data_hash).await? {
                None => None,
                Some(value) => {
                    self.cache_data(data_hash.to_vec(), value.clone()).await;
                    Some(value)
                }
            }),
        }
    }

    async fn insert_hash(
        &self,
        data_hash: Vec<u8>,
    ) -> Result<types::HashTable, ContainerError<Self::Error>> {
        let output = self.inner.insert_hash(data_hash.clone()).await?;
        self.cache_data(data_hash, output.clone()).await;
        Ok(output)
    }
}