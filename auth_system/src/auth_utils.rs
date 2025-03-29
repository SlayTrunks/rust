
pub mod models ;
pub fn login(cred:models::Credentials) {

    crate::database::get_user(); //also gives jwt tokens and things

}


        
