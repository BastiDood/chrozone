#![no_std]
extern crate alloc;

mod interaction;

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
        return Err(StatusCode::METHOD_NOT_ALLOWED);
    }

    if path != "/discord" {
        return Err(StatusCode::NOT_FOUND);
    }

    // Retrieve security headers
    let maybe_sig = headers.get("X-Signature-Ed25519");
    let maybe_time = headers.get("X-Signature-Timestamp");
    let (sig, timestamp) = maybe_sig.zip(maybe_time).ok_or(StatusCode::UNAUTHORIZED)?;
    let signature = hex::decode(sig).map_err(|_| StatusCode::BAD_REQUEST)?;

    // Append body after the timestamp
    let payload = hyper::body::to_bytes(body).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut message = timestamp.as_bytes().to_vec();
    message.extend_from_slice(&payload);

    // Validate the challenge
    pub_key.verify(&message, &signature).map_err(|_| StatusCode::UNAUTHORIZED)?;
    drop(message);
    drop(signature);

    // Parse incoming interaction
    let interaction = serde_json::from_slice(&payload).map_err(|_| StatusCode::BAD_REQUEST)?;
    drop(payload);

    let reply = interaction::respond(interaction);
    let body = serde_json::to_string(&reply).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.into();
    Ok(Response::new(body))
}
