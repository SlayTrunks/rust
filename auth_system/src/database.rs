
pub enum Db_Status {
    Connected,
    Interrupted
}
pub fn connect_to_db() -> Db_Status{
    //connect to db if connected return    
    Db_Status::Connected
}
pub fn get_user() {
    //fetch the user from db and return
}

