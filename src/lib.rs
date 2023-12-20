#![no_std]
extern crate alloc;

mod interaction;
mod util;

use http_body_util::Full;
use hyper::{
    body::{Bytes, Incoming},
    HeaderMap, Method, Response, StatusCode,
};
use ring::signature::UnparsedPublicKey;

pub fn from_err_status(code: StatusCode) -> Response<Full<Bytes>> {
    let mut res = Response::new(Full::new(Bytes::new()));
    *res.status_mut() = code;
    res
}

pub async fn try_respond<B>(
    mut body: Incoming,
    method: Method,
    path: &str,
    headers: &HeaderMap,
    pub_key: &UnparsedPublicKey<B>,
) -> core::result::Result<Response<Full<Bytes>>, StatusCode>
where
    B: AsRef<[u8]>,
{
    match method {
        Method::GET => {
            if path == "/" {
                log::info!("Health check pinged!");
                Ok(Response::new(Full::new(Bytes::new())))
            } else {
                log::error!("Invalid health check path: {path}");
                Err(StatusCode::NOT_FOUND)
            }
        }
        Method::POST => {
            if path != "/discord" {
                log::error!("Rejected invalid path request: {path}");
                return Err(StatusCode::NOT_FOUND);
            }

            // Retrieve security headers
            let maybe_sig = headers.get("X-Signature-Ed25519");
            let maybe_time = headers.get("X-Signature-Timestamp");
            let (sig, timestamp) = maybe_sig.zip(maybe_time).ok_or(StatusCode::UNAUTHORIZED)?;
            let signature = hex::decode(sig).map_err(|_| StatusCode::BAD_REQUEST)?;
            log::debug!("Timestamp and signature retrieved.");

            // Append body after the timestamp
            use http_body_util::BodyExt;
            let mut message = timestamp.as_bytes().to_vec();
            let start = message.len();
            while let Some(frame) = body.frame().await {
                let frame = match frame {
                    Ok(frame) => frame,
                    Err(err) => {
                        log::error!("body stream prematurely ended: {err}");
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                };
                if let Some(data) = frame.data_ref() {
                    message.extend_from_slice(data);
                }
            }
            log::debug!("Fully received payload body.");

            // Validate the challenge
            pub_key.verify(&message, &signature).map_err(|_| StatusCode::UNAUTHORIZED)?;
            drop(signature);
            log::debug!("Ed25519 signature verified.");

            // Parse incoming interaction
            let json = message.get(start..).ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
            let interaction = serde_json::from_slice(json).map_err(|_| StatusCode::BAD_REQUEST)?;
            log::debug!("Interaction JSON body parsed.");

            let reply = interaction::respond(interaction);
            let body = serde_json::to_string(&reply).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.into();

            use hyper::header::{HeaderValue, CONTENT_TYPE};
            let mut res = Response::new(body);
            res.headers_mut().append(CONTENT_TYPE, HeaderValue::from_static("application/json"));
            Ok(res)
        }
        _ => {
            log::error!("Rejected non-POST request: {method} {path}");
            Err(StatusCode::METHOD_NOT_ALLOWED)
        }
    }
}
