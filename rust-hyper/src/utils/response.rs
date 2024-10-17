use hyper::{Body, Response, StatusCode};

pub fn create_response(status: u16, body: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
        .body(Body::from(body.to_string()))
        .unwrap()
}
