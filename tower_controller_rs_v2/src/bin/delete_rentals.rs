use dotenv::dotenv;
use firestore::FirestoreDb;
use gcloud_sdk::google::firestore::v1::Document;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = FirestoreDb::new("fahrradturm")
        .await
        .expect("Failed to create FirestoreDb");

    let users = db.fluent().select().from("users").query().await.unwrap();

    let mut rentals: Vec<(String, Document)> = Vec::new();

    for user in users {
        let user_id = user.name.split('/').last().unwrap().to_owned();

        let user_rentals = db
            .fluent()
            .select()
            .from("rentals")
            .parent(db.parent_path("users", &user_id).unwrap())
            .query()
            .await
            .unwrap();

        rentals.append(
            &mut user_rentals
                .into_iter()
                .map(|r| (user_id.clone(), r))
                .collect::<Vec<(String, Document)>>(),
        );
    }

    for rental in rentals {
        let delete = db
            .fluent()
            .delete()
            .from("rentals")
            .document_id(rental.1.name.split('/').last().unwrap())
            .parent(db.parent_path("users", rental.0).unwrap())
            .execute()
            .await
            .unwrap();

        println!("deleted: {:?}", delete);
    }
}
