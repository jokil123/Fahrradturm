use std::{sync::Arc, time::Duration};

use firestore::FirestoreDb;
use gcloud_sdk::google::firestore::v1::listen_response::ResponseType;
use tokio::sync::Mutex;

use crate::{
    database::TowerDatabase,
    entities::firestore_job::{ConfirmType, FirestoreJob, JobError},
    tower::Tower,
    util::{box_id_to_coords, coords_to_box_id, timestamp_to_duration},
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

    println!("4: Got new document");

    let Ok(assignment) = FirestoreDb::deserialize_doc_to::<FirestoreJob>(&doc) else {
      db.set_error(doc.name.split("/").last().unwrap(), JobError::InvalidMessage).await?;
      return Ok(());
    };

    println!("5: Assignment deserialized");

    let id = assignment.id.as_ref().unwrap();

    let Ok(_) = db.has_subscription(&assignment).await else {
      db.set_error(id, JobError::InvalidPermissions).await?;
      return Ok(());
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
                    db.set_error(id, JobError::NoFreeSlots).await?;
                    return Ok(());
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
            let Some(box_id) = assignment.box_id else {
                db.set_error(id, JobError::NoSlotSpecified)
                    .await?;
                return Ok(());
            };

            let Ok(slot_location) = box_id_to_coords(&box_id) else {
                db.set_error(id, JobError::InvalidSlot).await?;
                return Ok(());
            };

            println!("10: Got slot location");

            if !tower.slot_exists(&slot_location)? {
                db.set_error(id, JobError::InvalidSlot).await?;
                return Ok(());
            }

            println!("11: Slot exists");

            if !tower.slot_rented_by_user(&slot_location, &assignment.user_id)? {
                db.set_error(id, JobError::InvalidPermissions).await?;
                return Ok(());
            }

            println!("12: Checked rental status");

            tower
                .retrieve_object(&slot_location, &assignment.user_id)
                .await?;

            println!("13: Retrieved");
        }
    }

    db.set_confirm(id, ConfirmType::JobCompleted).await?;

    println!("14: Confirmation sent");

    Ok(())
}
