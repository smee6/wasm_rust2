use hyper::{Body, Response, StatusCode};
use wasmtime::*;
use std::convert::Infallible;

pub async fn matrix_multiply() -> Result<Response<Body>, Infallible> {
    let engine = Engine::default();
    let module = match Module::from_file(&engine, "./wasms/matrix_multiply_wasm.wasm") {
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

    let matrix_multiply = match instance.get_typed_func::<(), f64, _>(&mut store, "matrix_multiply") {
        Ok(func) => func,
        Err(err) => {
            let body = format!("Failed to get Wasm function: {}", err);
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(body))
                .unwrap());
        }
    };

    let result = match matrix_multiply.call(&mut store, ()) {
        Ok(result) => result,
        Err(err) => {
            let body = format!("Failed to execute Wasm function: {}", err);
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(body))
                .unwrap());
        }
    };

    let body = format!("Result: {}", result);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))
        .unwrap())
}
