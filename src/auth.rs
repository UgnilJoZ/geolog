use crate::database::Database;
use crate::errors::Error;
use crate::types::{Device, DeviceToken};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;

/// Strips a prefix from a byte sequence
///
/// If `data` starts with `prefix`, returns Some(the remaining sequence).
///
/// If `data` does not start with this prefix, returns None.
fn strip_prefix<'a>(data: &'a [u8], prefix: &[u8]) -> Option<&'a [u8]> {
    let mut remaining = data;
    for prefix_char in prefix {
        if prefix_char == remaining.first()? {
            remaining = &remaining[1..];
        } else {
            return None;
        }
    }
    Some(remaining)
}

impl Device {
    /// Returns an authenticated device
    ///
    /// If the http request contains a proper device authentication via Authorization header,
    /// the device will be returned. Else, the reason why the authentication failed is returned.
    async fn from_authentication(req: HttpRequest) -> Result<Self, Error> {
        let header_value = req
            .headers()
            .get("Authorization")
            .ok_or(Error::Unauthenticated)?
            .as_bytes();
        let token_string =
            strip_prefix(header_value, b"Token ").ok_or(Error::AuthHeaderMalformed)?;
        let token = DeviceToken::from_base64(token_string).map_err(Error::Base64DecodingError)?;
        let db = req
            .app_data::<Data<Database>>()
            .ok_or(Error::DatabaseNotPresent)?;
        Ok(db.get_device(token).await?)
    }
}

impl FromRequest for Device {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move { Self::from_authentication(req).await })
    }
}
