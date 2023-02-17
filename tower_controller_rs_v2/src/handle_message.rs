use std::{sync::Arc, time::Duration};

use firestore::FirestoreDb;
use gcloud_sdk::google::firestore::v1::listen_response::ResponseType;

use crate::{
    controller_error::ControllerError,
    database::{ConfirmType, TowerDatabase},
    entities::firestore_assignment::{AssignmentError, FirestoreAssignment},
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

    println!("4: Got new document");

    let db = db.lock().await;

    let Ok(assignment) = FirestoreDb::deserialize_doc_to::<FirestoreAssignment>(&doc) else {
      db.set_error(doc.name.split("/").last().unwrap(), AssignmentError::InvalidMessage).await;
      return Ok(());
    };

    println!("5: Assignment deserialized");

    let id = assignment.id.as_ref().unwrap();

    let Ok(_) = db.check_permissions(&assignment).await else {
      db.set_error(id, AssignmentError::InvalidPermissions).await;
      return Ok(());
    };

    println!("6: Permissions checked");

    db.set_confirm(id, ConfirmType::JobRecieved).await;

    println!("7: Confirmation sent");

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

            println!("8: Found free slot");

            tower.store(slot).unwrap();

            println!("9: Stored");
        }
        AssignmentType::Retrieve => {
            let Some(slot_location) = assignment.slot else {
                db.set_error(id, AssignmentError::NoSlotSpecified)
                    .await;
                return Ok(());
            };

            println!("10: Got slot location");

            let Ok(slot) = tower.get_slot(slot_location) else {
                db.set_error(id, AssignmentError::InvalidSlot)
                    .await;
                return Ok(());
            };

            println!("11: Got slot");

            if slot.rental_status != RentalStatus::Rented(assignment.user_id) {
                db.set_error(id, AssignmentError::InvalidPermissions).await;
                return Ok(());
            }

            println!("12: Checked rental status");

            tower.retrieve(slot).unwrap();

            println!("13: Retrieved");
        }
    }

    db.set_confirm(id, ConfirmType::JobCompleted).await;

    println!("14: Confirmation sent");

    Ok(())
}
