use dotenv::dotenv;
use firestore::{struct_path::paths, FirestoreDb};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = FirestoreDb::new("fahrradturm").await.unwrap();

    let changed_doc = db
        .fluent()
        .update()
        .fields(paths!(MyStruct::my_value))
        .in_col("myCollection")
        .document_id("myDocument")
        .object(&MyStruct {
            my_value: 42,
            ..Default::default()
        })
        .execute::<MyStruct>()
        .await
        .unwrap();

    println!("{:?}", changed_doc);
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct MyStruct {
    #[serde(rename = "myValue")]
    my_value: i32,
    #[serde(rename = "myOtherValue")]
    my_other_value: i32,
}
