use axum::{
    body::{Body, HttpBody},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

macro_rules! mid {
    ($path:expr, $method:expr; $($cnd:expr),+) => {
        Router::new()
            .route($path, $method)
            .route_layer(middleware::from_fn(|r, n| layer(r, n, &[$($cnd),+])))
    };
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(mid!("/", get(|| async { "tyler\n" }); "tyler"))
        .merge(mid!("/cow", get(|| async { "cow\n" }); "cow"))
        .merge(mid!("/h", get(|| async {"h h h\n"}); "gregory", "power", "tyler"));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn layer(req: Request<Body>, next: Next<Body>, cnd: &[&str]) -> Result<Response, StatusCode> {
    println!("{cnd:?}");
    if cnd.contains(&"tyler") {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::IM_A_TEAPOT)
    }
}
