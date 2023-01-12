use async_trait::async_trait;
use firestore::*;
use rvstruct::ValueStruct;

// The file where we store the cursor/token for the event when we read the last time
const RESUME_TOKEN_FILENAME: &str = "last-read-token";

#[derive(Clone)]
pub struct TempFileTokenStorage;

#[async_trait]
impl FirestoreResumeStateStorage for TempFileTokenStorage {
    async fn read_resume_state(
        &self,
        target: &FirestoreListenerTarget,
    ) -> Result<Option<FirestoreListenerTargetResumeType>, Box<dyn std::error::Error + Send + Sync>>
    {
        let target_state_file_name = format!("{}.{}.tmp", RESUME_TOKEN_FILENAME, target.value());
        let token = std::fs::read_to_string(target_state_file_name)
            .ok()
            .map(|str| {
                hex::decode(&str)
                    .map(FirestoreListenerToken::new)
                    .map(FirestoreListenerTargetResumeType::Token)
                    .map_err(|e| Box::new(e))
            })
            .transpose()?;

        Ok(token)
    }

    async fn update_resume_token(
        &self,
        target: &FirestoreListenerTarget,
        token: FirestoreListenerToken,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let target_state_file_name = format!("{}.{}.tmp", RESUME_TOKEN_FILENAME, target.value());

        Ok(std::fs::write(
            target_state_file_name,
            hex::encode(token.value()),
        )?)
    }
}
