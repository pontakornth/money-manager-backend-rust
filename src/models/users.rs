use serde::{Serialize,Deserialize};
use bcrypt::{hash,verify,DEFAULT_COST};
use std::collections::HashMap;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Error;

#[derive(Serialize,Deserialize)]
pub struct User {
   pub username: String,
   pub display_name: String
}

pub enum UserError {
   UsernameAlreadyExist,
   WrongPassword,
   EmptyUsername,
   UserDoesNotExist
}

fn generate_password(raw_password: &String) -> String {
   hash(raw_password, DEFAULT_COST).unwrap()
}

fn verify_password(raw_password: &String,hash: &String) -> bool {
   verify(raw_password, &hash).unwrap()
}

fn register(user_data: &HashMap<String,String>) -> Result<(),UserError> {
   let mock_username = vec![String::from("mmknightx"),String::from("kenzie")];
   let username = &user_data.get("username").unwrap();
   if mock_username.contains(username) {
      Err(UserError::UsernameAlreadyExist)
   } else if username.len() == 0 {
      Err(UserError::EmptyUsername)
   } else {
      Ok(())
   }
}

pub fn get_user_by_username(db: &PooledConnection<SqliteConnectionManager>, username: &String) -> Result<User,Error> {
   db.query_row(
      "SELECT username, display_name FROM users WHERE username=?1", &[&username], |row| {
         Ok(
             User {
                username: row.get::<_,String>(0).unwrap(),
                display_name: row.get::<_,String>(1).unwrap()
            }
         )
      }
   )
}

pub fn get_user_by_id(db: &PooledConnection<SqliteConnectionManager>, id: i64) -> Result<User,Error> {
   db.query_row(
      "SELECT username, display_name FROM users WHERE id=?1", &[id], |row| {
         println!("Id is {}",&id);
         Ok(
             User {
                username: row.get::<_,String>(0).unwrap(),
                display_name: row.get::<_,String>(1).unwrap()
           }
         )
      }
   )
}
