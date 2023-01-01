use axum::routing::{get, post, put};


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::debug!("Logging initialized!");

    log::debug!("Configuring Axum router...");

    let webapp = axum::Router::new()
        // .route("/", get(routes::home::route_home_get))
        .layer(tower_http::cors::CorsLayer::new()
            .allow_origin("*".parse::<axum::http::HeaderValue>().expect("* to be a valid origin"))
        );

    log::info!("Starting Axum server...");

    axum::Server::bind(&config::AXUM_HOST).serve(webapp.into_make_service()).await
        .expect("to be able to run the Axum server");
}
