use pam::{Authenticator};
use serde::Deserialize;
use jsonrpc_v2::{Data, Error};
use std::sync::atomic::{AtomicBool, Ordering};
use actix_web::{get, HttpResponse, Responder, http::StatusCode};
use std::env;

static STATUSVAR: AtomicBool = AtomicBool::new(false);


#[derive(Deserialize)]
pub struct InputUser {
    pub username: String,
    pub password: String,
}

pub fn set_status(newstatus: bool) {
    STATUSVAR.store(newstatus, Ordering::Relaxed);
}

pub fn get_status() -> bool {
    STATUSVAR.load(Ordering::Relaxed)
}

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../sites/404.html"))
}


pub async fn privateapi() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../sites/GraphiQL.html"))
}


pub async fn pam_login(logindata: Data<InputUser>) -> Result<(), Error> {
    
    let error_enum = Error::Provided{code: 404, message: "Your Username password is not correct"};

    let newlogin = InputUser{
        username: String::from(logindata.username.as_str()),
        password: String::from(logindata.password.as_str()),
    };

    // setup authenticator with system-auth
    let service = "system-auth";
    let mut auth = Authenticator::with_password(&service)
        .unwrap();

    // Now, give username password to be authenticated 
    auth.get_handler()
        .set_credentials(&newlogin.username, &newlogin.password);

    // Now, Authenticate and Listen for feedback
    if  auth.authenticate()
            .is_ok() && 
        auth
            .open_session()
            .is_ok() {
        set_status(true);
        // env::set_var("RUST_TEST", format!("{}", get_status()));
        Ok(())
    }
    else{
        Err(error_enum)
    }
}

