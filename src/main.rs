use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

macro_rules! db {
    ($path:expr, $method:expr; $($cnd:expr),+) => {
        Router::new()
            .route($path, $method)
            .route_layer(middleware::from_fn(|r, n| layer(r, n, &[$($cnd),+])))
    };
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(db!("/", get(|| async { "tyler\n" }); "tyler"))
        .merge(db!("/cow", get(|| async { "cow\n" }); "cow"))
        .merge(db!("/h", get(|| async {"h h h\n"}); "gregory", "power", "tyler"));

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
