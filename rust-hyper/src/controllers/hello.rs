use hyper::{Body, Response, StatusCode};
use wasmtime::*;
use std::convert::Infallible;

pub async fn hello_world() -> Result<Response<Body>, Infallible> {
    let engine = Engine::default();
    let module = match Module::from_file(&engine, "./wasms/hello_world_wasm.wasm") {
        Ok(module) => module,
        Err(err) => {
            let body = format!("Failed to load Wasm module: {}", err);
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(body))
                .unwrap());
        }
    };

    let mut store = Store::new(&engine, ());
    let instance = match Instance::new(&mut store, &module, &[]) {
        Ok(instance) => instance,
        Err(err) => {
            let body = format!("Failed to create Wasm instance: {}", err);
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(body))
                .unwrap());
        }
    };

    let hello_function = match instance.get_typed_func::<(), i32, _>(&mut store, "hello_world") {
        Ok(func) => func,
        Err(err) => {
            let body = format!("Failed to get Wasm function: {}", err);
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(body))
                .unwrap());
        }
    };

    let result = match hello_function.call(&mut store, ()) {
        Ok(result) => result,
        Err(err) => {
            let body = format!("Failed to execute Wasm function: {}", err);
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(body))
                .unwrap());
        }
    };

    let body = format!("Hello, World! Returned value: {}", result);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))
        .unwrap())
}
