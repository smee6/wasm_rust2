use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::convert::Infallible;
use tokio::time::Instant;
use wasmtime::*;
use tokio::task;

async fn handle_hello() -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello, world!")))
}

async fn handle_bye() -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Goodbye, world!")))
}

async fn handle_mirror(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    Ok(Response::new(Body::from(full_body)))
}

fn factorize(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2;
    let mut num = n;
    while num > 1 {
        while num % d == 0 {
            factors.push(d);
            num /= d;
        }
        d += 1;
        if d * d > num {
            if num > 1 {
                factors.push(num);
            }
            break;
        }
    }
    factors
}

async fn factory_caller() -> Result<Response<Body>, Infallible> {
    let start_time = Instant::now();

    for _ in 0..1000 {
        factorize(10_000_000_000_031);
    }

    let duration = start_time.elapsed();
    let body = format!("Completed in: {:?}", duration);
    Ok(Response::new(Body::from(body)))
}

async fn factory_caller2() -> Result<Response<Body>, Infallible> {
    let start_time = Instant::now();

    // Initialize Wasmtime
    let engine = Engine::default();
    let module = Module::from_file(&engine, "factorize_wasm.wasm").unwrap();
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).unwrap();
    let factorize = instance
        .get_typed_func::<u64, u64, _>(&mut store, "factorize")
        .unwrap();

    // Call the Wasm module's factorize function
    for _ in 0..1000 {
        let _ = factorize.call(&mut store, 10_000_000_000_031).unwrap();
    }

    let duration = start_time.elapsed();
    let body = format!("Completed in: {:?}", duration);
    Ok(Response::new(Body::from(body)))
}

async fn factory_caller3() -> Result<Response<Body>, Infallible> {
    let start_time = Instant::now();

    // Wasmtime 초기화
    let engine = Engine::default();
    let module = Module::from_file(&engine, "matrix_multiply_wasm.wasm").unwrap(); // 새로운 Wasm 모듈
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).unwrap();

    let matrix_multiply = instance
        .get_typed_func::<(), f64, _>(&mut store, "matrix_multiply")
        .unwrap();

    // Wasm 모듈의 matrix_multiply 함수 호출
    let result = matrix_multiply.call(&mut store, ()).unwrap();

    let duration = start_time.elapsed();
    let body = format!("Completed in: {:?}, Result: {}", duration, result);
    Ok(Response::new(Body::from(body)))
}

async fn matrix_multiply() -> Result<Response<Body>, Infallible> {
    let start_time = Instant::now();
    const N: usize = 1000;

    // Use heap-allocated matrices
    let a = vec![vec![1.0; N]; N];
    let b = vec![vec![2.0; N]; N];
    let mut c = vec![vec![0.0; N]; N];

    // Matrix multiplication using parallelism
    task::spawn_blocking(move || {
        for i in 0..N {
            for j in 0..N {
                let mut sum = 0.0;
                for k in 0..N {
                    sum += a[i][k] * b[k][j];
                }
                c[i][j] = sum;
            }
        }
    }).await.unwrap();

    // Return the first element of the result matrix

    let duration = start_time.elapsed();
    let body = format!("Completed in: {:?}", duration);
    Ok(Response::new(Body::from(body)))
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/hello") => handle_hello().await,
        (&Method::GET, "/bye") => handle_bye().await,
        (&Method::POST, "/mirror") => handle_mirror(req).await,
        (&Method::GET, "/factorize") => factory_caller().await,
        (&Method::GET, "/factorize2") => factory_caller2().await,
        (&Method::GET, "/matrix_multiply") => factory_caller3().await,
        (&Method::GET, "/matrix_multiply2") => matrix_multiply().await,
        _ => Ok(Response::new(Body::from("Not Found"))),
    }
}

#[tokio::main]
async fn main() {
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
