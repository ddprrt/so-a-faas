use now_lambda::lambda;
use std::error::Error;

use http::{Request, Response, StatusCode};

fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let uri = request.uri();
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(format!("You made a request to the Rust Lambda on: {}", uri))
        .expect("failed to render response");

    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
