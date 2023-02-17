use std::{sync::Arc, time::Duration};

use firestore::FirestoreDb;
use gcloud_sdk::google::firestore::v1::listen_response::ResponseType;

use crate::{
    database::TowerDatabase,
    entities::firestore_assignment::{AssignmentError, ConfirmType, FirestoreAssignment},
    tower::Tower,
    util::timestamp_to_duration,
};

use crate::entities::firestore_assignment::AssignmentType;

pub async fn handle_message(
    response: ResponseType,
    start_time: Duration,
    db: Arc<tokio::sync::Mutex<TowerDatabase>>,
    tower: Arc<tokio::sync::Mutex<Tower>>,
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

    let db = db.lock().await;

    let Ok(assignment) = FirestoreDb::deserialize_doc_to::<FirestoreAssignment>(&doc) else {
      db.set_error(doc.name.split("/").last().unwrap(), AssignmentError::InvalidMessage).await?;
      return Ok(());
    };

    println!("5: Assignment deserialized");

    let id = assignment.id.as_ref().unwrap();

    let Ok(_) = db.has_subscription(&assignment).await else {
      db.set_error(id, AssignmentError::InvalidPermissions).await?;
      return Ok(());
    };

    println!("6: Permissions checked");

    db.set_confirm(id, ConfirmType::JobRecieved).await?;

    println!("7: Confirmation sent");

    let mut tower = tower.lock().await;

    match assignment.assignment_type {
        AssignmentType::Store => {
            let slot_location = match tower.find_free_slot() {
                Ok(slot) => slot,
                Err(e) => {
                    db.set_error(id, AssignmentError::NoFreeSlots).await?;
                    return Ok(());
                }
            };

            println!("8: Found free slot");

            tower.store(&slot_location).unwrap();

            db.set_slot(id, &slot_location).await?;

            println!("9: Stored");
        }
        AssignmentType::Retrieve => {
            let Some(slot_location) = assignment.slot else {
                db.set_error(id, AssignmentError::NoSlotSpecified)
                    .await?;
                return Ok(());
            };

            println!("10: Got slot location");

            if !tower.slot_exists(&slot_location)? {
                db.set_error(id, AssignmentError::InvalidSlot).await?;
                return Ok(());
            }

            println!("11: Slot exists");

            if !tower.slot_rented_by_user(&slot_location, &assignment.user_id)? {
                db.set_error(id, AssignmentError::InvalidPermissions)
                    .await?;
                return Ok(());
            }

            println!("12: Checked rental status");

            tower.retrieve(&slot_location)?;

            println!("13: Retrieved");
        }
    }

    db.set_confirm(id, ConfirmType::JobCompleted).await?;

    println!("14: Confirmation sent");

    Ok(())
}
