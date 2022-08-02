pub extern crate crypto;
pub extern crate jwt;
pub extern crate rustc_serialize;

use admin::auth::crypto::sha2::Sha256;
use self::jwt::{
    Header,
    Registered,
    Token,
};


pub struct ApiKey(pub String);

pub fn readToken(token: &str) -> Option<ApiKey> {
    let token = Token::<Header, Registered>::parse(token).ok()?;
    let key = token.secret().to_string();
    Some(ApiKey(key))
}

impl<'a, 'b> From<&'a str> for ApiKey {
    type Error = ();

    fn fromRequest(req: &'a Request<'b>) -> Result<Self, Self::Error> {
        let token = req.headers.get("Authorization").ok()?;
        let token = token.to_str().ok()?;
        let token = readToken(token)?;
        Ok(token)
    }
}