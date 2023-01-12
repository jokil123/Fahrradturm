use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use firestore::*;
use rvstruct::ValueStruct;

#[derive(Clone)]
pub struct HashMapTokenStorage {
    map: Arc<RwLock<HashMap<i32, Vec<u8>>>>,
}

#[async_trait]
impl FirestoreResumeStateStorage for HashMapTokenStorage {
    async fn read_resume_state(
        &self,
        target: &FirestoreListenerTarget,
    ) -> Result<Option<FirestoreListenerTargetResumeType>, Box<dyn std::error::Error + Send + Sync>>
    {
        let map_lock = self.map.read().;

        map_lock
            .get(target.value())
            .map(|str| FirestoreListenerToken::new(str.to_owned()))
            .map(FirestoreListenerTargetResumeType::Token);

        // let target_state_file_name = format!("{}.{}.tmp", RESUME_TOKEN_FILENAME, target.value());
        // let token = std::fs::read_to_string(target_state_file_name)
        //     .ok()
        //     .map(|str| {
        //         hex::decode(&str)
        //             .map(FirestoreListenerToken::new)
        //             .map(FirestoreListenerTargetResumeType::Token)
        //             .map_err(|e| Box::new(e))
        //     })
        //     .transpose()?;

        // Ok(token)
    }

    async fn update_resume_token(
        &self,
        target: &FirestoreListenerTarget,
        token: FirestoreListenerToken,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())

        // let target_state_file_name = format!("{}.{}.tmp", RESUME_TOKEN_FILENAME, target.value());

        // Ok(std::fs::write(
        //     target_state_file_name,
        //     hex::encode(token.value()),
        // )?)
    }
}
