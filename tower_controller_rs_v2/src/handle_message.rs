use std::{sync::Arc, time::Duration};

use firestore::FirestoreDb;
use gcloud_sdk::google::firestore::v1::listen_response::ResponseType;

use crate::{
    controller_error::ControllerError,
    database::{ConfirmType, TowerDatabase},
    entities::firestore_assignment::{Assignment, AssignmentError},
    tower::{RentalStatus, Tower},
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

    if doc.create_time != doc.update_time {
        return Ok(());
    }

    let db = db.lock().await;

    let Ok(assignment) = FirestoreDb::deserialize_doc_to::<Assignment>(&doc) else {
      db.set_error(doc.name.split("/").last().unwrap(), AssignmentError::InvalidMessage).await;
      return Ok(());
    };

    let id = assignment.id.as_str();

    let Ok(_) = db.check_permissions(&assignment).await else {
      db.set_error(id, AssignmentError::InvalidPermissions).await;
      return Ok(());
    };

    db.set_confirm(id, ConfirmType::JobRecieved).await;

    let mut tower = tower.lock().await;

    match assignment.assignment_type {
        AssignmentType::Store => {
            let slot = match tower.find_free_slot() {
                Ok(slot) => slot,
                Err(e) => {
                    db.set_error(id, AssignmentError::NoFreeSlots).await;
                    return Ok(());
                }
            };

            tower.store(slot);
        }
        AssignmentType::Retrieve => {
            let Some(slot_location) = assignment.slot else {
                db.set_error(id, AssignmentError::NoSlotSpecified)
                    .await;
                return Ok(());
            };

            let Ok(slot) = tower.get_slot(slot_location) else {
                db.set_error(id, AssignmentError::InvalidSlot)
                    .await;
                return Ok(());
            };

            if slot.rental_status != RentalStatus::Rented(assignment.user_id) {
                db.set_error(id, AssignmentError::InvalidPermissions).await;
                return Ok(());
            }

            tower.retrieve(slot).unwrap();
        }
    }

    db.set_confirm(id, ConfirmType::JobCompleted).await;

    Ok(())
}
