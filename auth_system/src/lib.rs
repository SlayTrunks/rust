
#![allow(dead_code,unused_variables)]
use crate::database::Db_Status;
pub mod database;
pub mod auth_utils;
pub fn authenticate(cred:crate::auth_utils::models::Credentials) {

    //    if let Db_Status::Connected = connect_to_db(){
    //
    //    
    //    }
    match crate::database::connect_to_db()  {
        Db_Status::Connected => crate::auth_utils::login(cred),
        Db_Status::Interrupted => () //returns nothing
    }
}
