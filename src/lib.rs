use hyper::{Body, Request, Response, StatusCode};

pub fn from_err_status(code: StatusCode) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    *res.status_mut() = code;
    res
}

pub fn try_respond(req: Request<Body>, pub_key: &[u8]) -> Result<Response<Body>, StatusCode> {
    todo!()
}
