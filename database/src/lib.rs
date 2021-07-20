use mongodb::{bson::doc, sync::Client, sync::Database};

fn get_connection() -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(
        "mongodb://localhost:27017",
    )?;

    let db = client.database("ZooDB");

    Ok(db)
}

pub fn get_animal() {
    let database = get_connection();
    match database {
        Ok(db) => {
            let list = db.list_collection_names(None);
            match list {
                Ok(raw) => println!{"{:?}", raw},
                Err(e) => println!("error: {:?}", e),
            }},
        Err(e) => println!("error: {:?}", e),
    }
}