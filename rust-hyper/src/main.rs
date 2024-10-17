use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::convert::Infallible;
use dotenv::dotenv;
use std::env;

mod controllers;
mod middleware;
mod utils;

use controllers::matrix::matrix_multiply;
use controllers::hello::hello_world;
use middleware::guard::query_checker;
use utils::response::create_response;

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.uri().path() == "/matrix_multiply" {
        if let Err(response) = query_checker(&req).await {
            return Ok(response);
        }
    }
    
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/matrix_multiply") => matrix_multiply().await,
        (&Method::GET, "/hello") => hello_world().await,
        _ => Ok(create_response(404, "Not Found")),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let server_ip = env::var("SERVER_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid u16 number");

    let addr = (server_ip.parse::<std::net::IpAddr>().expect("Invalid IP address"), server_port).into();
    
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router)) });
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}