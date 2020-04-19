use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct User {
   pub username: String,
   pub display_name: String
}

