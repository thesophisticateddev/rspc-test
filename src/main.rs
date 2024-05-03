use axum::routing::get;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let router = rspc::Router::<()>::new()
        .query("version", |t| t(|_, _: ()| "1.0.0"))
        .build()
        .arced();

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest("/rspc", rspc_axum::endpoint(router.clone(), || ()));
    // let addr = "[::]:9000".parse::<SocketAddr>().unwrap(); // This listens on IPv6 and IPv4
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    // println!("{} listening on http://{}", env!("CARGO_CRATE_NAME"), addr);
    axum::serve(listener, app).await.unwrap();
}
