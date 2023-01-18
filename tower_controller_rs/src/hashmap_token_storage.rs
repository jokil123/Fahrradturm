use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use firestore::*;
use rvstruct::ValueStruct;

pub struct HashMapTokenStorage {
    storage: Arc<RwLock<HashMap<FirestoreListenerTarget, FirestoreListenerToken>>>,
}

#[async_trait]
impl FirestoreResumeStateStorage for HashMapTokenStorage {
    async fn read_resume_state(
        &self,
        target: &FirestoreListenerTarget,
    ) -> Result<Option<FirestoreListenerTargetResumeType>, Box<dyn std::error::Error + Send + Sync>>
    {
        let storage = self.storage.read().expect("Failed to (read)lock storage");

        Ok(storage
            .get(target)
            .cloned()
            .map(FirestoreListenerTargetResumeType::Token))
    }

    async fn update_resume_token(
        &self,
        target: &FirestoreListenerTarget,
        token: FirestoreListenerToken,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut storage = self.storage.write().expect("Failed to (write)lock storage");
        storage.insert(target.clone(), token);
        Ok(())
    }
}
