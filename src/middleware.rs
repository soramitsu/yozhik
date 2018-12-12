use actix_web::{HttpRequest, HttpMessage, Result};
use actix_web::error::{ErrorUnauthorized, ParseError};

use sha1::Sha1;
use hmac::{Hmac, Mac};
use hex::encode;

use super::config::WEBHOOK_KEY;

pub fn validate(req: &HttpRequest<()>, body: &[u8]) -> Result<()> {
    let sig: &str = req.headers()
        .get("X-Hub-Signature")
        .ok_or(ErrorUnauthorized(ParseError::Header))?
        .to_str()
        .map_err(ErrorUnauthorized)?;

    let (_, sig) = sig.split_at(5); // strip "sha1=" from the header
    let sig = sig.to_string();

    let mut hmac = Hmac::<Sha1>::new_varkey(WEBHOOK_KEY.as_bytes()).unwrap();
    hmac.input(body);
    let code = encode(hmac.result().code());
    if code == sig {
        Ok(())
    } else {
        Err(ErrorUnauthorized(ParseError::Header))
    }
}
