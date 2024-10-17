use hyper::{Request, Response, Body, StatusCode};

pub async fn query_checker(req: &Request<Body>) -> Result<(), Response<Body>> {
    // Query 파라미터 검사 (예: "param"이 숫자인지 확인)
    if let Some(query) = req.uri().query() {
        let params: Vec<&str> = query.split('&').collect();
        for param in params {
            if let Some((key, value)) = param.split_once('=') {
                if key == "param" && value.parse::<i32>().is_ok() {
                    return Ok(());
                }
            }
        }
    }

    // 조건이 맞지 않으면 404 응답 반환
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Query parameter 'param' must be a valid number"))
        .unwrap();
    Err(response)
}
