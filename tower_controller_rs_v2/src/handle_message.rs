use std::{sync::Arc, time::Duration};

use firestore::FirestoreDb;
use gcloud_sdk::google::firestore::v1::{listen_response::ResponseType, Document};
use tokio::sync::Mutex;

use crate::{
    controller_error::ControllerError,
    database::TowerDatabase,
    entities::firestore_job::{ConfirmType, FirestoreJob, JobError},
    tower::Tower,
    util::{box_id_to_coords, timestamp_to_duration},
};

use crate::entities::firestore_job::JobType;

pub async fn handle_message(
    response: ResponseType,
    start_time: Duration,
    db: Arc<TowerDatabase>,
    tower: Arc<Mutex<Tower>>,
) -> std::result::Result<(), Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    let ResponseType::DocumentChange(c) = response else {
        return Ok(());
    };

    let Some(doc) = c.document else {
        return Ok(());
    };

    if timestamp_to_duration(doc.update_time.clone().unwrap()) < start_time {
        return Ok(());
    }

    if doc.create_time != doc.update_time {
        return Ok(());
    }

    match run_job(doc.clone(), db.clone(), tower).await {
        Ok(con) => {
            db.set_confirm(doc.name.split("/").last().unwrap(), con)
                .await?
        }
        Err(err) => {
            db.set_error(doc.name.split("/").last().unwrap(), err)
                .await?
        }
    }

    Ok(())
}

async fn run_job(
    doc: Document,
    db: Arc<TowerDatabase>,
    tower: Arc<Mutex<Tower>>,
) -> Result<ConfirmType, ControllerError> {
    let Ok(assignment) = FirestoreDb::deserialize_doc_to::<FirestoreJob>(&doc) else {
      return Err(ControllerError::InvalidMessage);
    };

    println!("5: Assignment deserialized");

    let id = assignment.id.as_ref().unwrap();

    let Ok(_) = db.has_subscription(&assignment).await else {
      return Err(ControllerError::InvalidPermissions);
    };

    println!("6: Permissions checked");

    db.set_confirm(id, ConfirmType::JobRecieved).await?;

    println!("7: Confirmation sent");

    let mut tower = tower.lock().await;

    match assignment.assignment_type {
        JobType::Store => {
            let slot_location = match tower.find_free_slot() {
                Ok(slot) => slot,
                Err(e) => {
                    return Err(ControllerError::NoFreeSlots);
                }
            };

            println!("8: Found free slot");

            tower
                .store_object(&slot_location, &assignment.user_id)
                .await?;

            println!("9: Stored");

            db.set_slot(id, &slot_location).await?;

            println!("9: Slot set");
        }
        JobType::Retrieve => {
            println!("{:#?}", assignment);

            let Some(box_id) = assignment.box_id else {
                return Err(ControllerError::NoSlotSpecified);
            };

            println!("10: Got box id");

            let Ok(slot_location) = box_id_to_coords(&box_id) else {
                return Err(ControllerError::InvalidSlot);
            };

            println!("10: Got slot location");

            if !tower.slot_exists(&slot_location)? {
                return Err(ControllerError::InvalidSlot);
            }

            println!("11: Slot exists");

            if !tower.slot_rented_by_user(&slot_location, &assignment.user_id)? {
                return Err(ControllerError::InvalidPermissions);
            }

            println!("12: Checked rental status");

            tower
                .retrieve_object(&slot_location, &assignment.user_id)
                .await?;

            println!("13: Retrieved");
        }
    }

    Ok(ConfirmType::JobCompleted)
}
