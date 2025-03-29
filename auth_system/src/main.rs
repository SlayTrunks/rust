use auth_system::{authenticate,Credentials};
fn main() {

    let cred = Credentials {
        username: String::from("insanedai") ,
        password: String::from("heroinsane") 
    };
    authenticate(cred);
}
