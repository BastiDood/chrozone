#![no_std]
extern crate alloc;

mod interaction;
mod util;

use hyper::{Body, HeaderMap, Method, Response, StatusCode};
use ring::signature::UnparsedPublicKey;

pub fn from_err_status(code: StatusCode) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    *res.status_mut() = code;
    res
}

pub async fn try_respond<B>(
    body: Body,
    method: Method,
    path: &str,
    headers: &HeaderMap,
    pub_key: &UnparsedPublicKey<B>,
) -> core::result::Result<Response<Body>, StatusCode>
where
    B: AsRef<[u8]>,
{
    if method != Method::POST {
        log::error!("Rejected non-POST request: {method} {path}");
        return Err(StatusCode::METHOD_NOT_ALLOWED);
    }

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
    let payload = hyper::body::to_bytes(body).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut message = timestamp.as_bytes().to_vec();
    message.extend_from_slice(&payload);
    log::debug!("Fully received payload body.");

    // Validate the challenge
    pub_key.verify(&message, &signature).map_err(|_| StatusCode::UNAUTHORIZED)?;
    drop(message);
    drop(signature);
    log::debug!("Ed25519 signature verified.");

    // Parse incoming interaction
    let interaction = serde_json::from_slice(&payload).map_err(|_| StatusCode::BAD_REQUEST)?;
    drop(payload);
    log::debug!("Interaction JSON body parsed.");

    let reply = interaction::respond(interaction);
    let body = serde_json::to_string(&reply).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.into();

    use hyper::header::{HeaderValue, CONTENT_TYPE};
    let mut res = Response::new(body);
    res.headers_mut().append(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(res)
}
