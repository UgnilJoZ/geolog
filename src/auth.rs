use crate::database::Database;
use crate::errors::Error;
use crate::types::{Device, DeviceToken};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;

fn strip_prefix<'a>(ascii_str: &'a [u8], prefix: &[u8]) -> Option<&'a [u8]> {
    let mut remaining = ascii_str;
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
    async fn from_request_async(req: HttpRequest) -> Result<Self, Error> {
        let header_value = req
            .headers()
            .get("Authorization")
            .ok_or(Error::Unauthenticated)?;
        let token_string =
            strip_prefix(header_value.as_bytes(), b"Token ").ok_or(Error::AuthHeaderMalformed)?;
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
        Box::pin(async move { Self::from_request_async(req).await })
    }
}
