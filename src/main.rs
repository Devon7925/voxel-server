mod args;
mod error;
mod signaling;
mod shared;

use crate::signaling::{ws_handler, ServerState};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router, Json};
use cgmath::{Vector3, Point3};
use clap::Parser;
use shared::{Lobby, GameSettings};
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};
use tracing_subscriber::prelude::*;

pub use args::Args;
pub use matchbox_protocol::PeerId;

#[tokio::main]
async fn main() {
    // Initialize logger
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "matchbox_server=info,tower_http=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_file(false)
                .with_target(false),
        )
        .init();

    // Parse clap arguments
    let args = Args::parse();

    // Setup router
    let server_state = Arc::new(futures::lock::Mutex::new(ServerState::default()));
    let app = Router::new()
        .route("/lobby_list", get(lobby_lister))
        .route("/health", get(health_handler))
        .route("/", get(ws_handler))
        .route("/:room_id", get(ws_handler))
        .layer(
            // Allow requests from anywhere - Not ideal for production!
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(
            // Middleware for logging from tower-http
            TraceLayer::new_for_http().on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .latency_unit(LatencyUnit::Micros),
            ),
        )
        .with_state(server_state);

    // Run server
    info!("Matchbox Signaling Server: {}", args.host);
    axum::Server::bind(&args.host)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Unable to run signaling server, is it already running?");
}

pub async fn health_handler() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn lobby_lister() -> impl IntoResponse {
    // insert your application logic here
    let lobbies = vec![
        Lobby {
            name: "Regular Multiplayer".to_string(),
            lobby_id: 1,
            settings: GameSettings {
                name: "Standard Open Multiplayer".to_string(),
                delta_time: 0.03,
                is_remote: true,
                player_count: 2,
                render_size: Vector3::new(32, 16, 32),
                spawn_location: Point3::new(10000.0, 1810.0, 10000.0),
                max_loaded_chunks: 2048,
                world_gen: shared::WorldGenSettings::Normal,
                fixed_center: false,
            },
        },
        Lobby {
            name: "Public Practice Range".to_string(),
            lobby_id: 2,
            settings: GameSettings {
                name: "Public Practice Range".to_string(),
                delta_time: 0.03,
                is_remote: true,
                player_count: 2,
                render_size: Vector3::new(32, 16, 32),
                spawn_location: Point3::new(10000.0, 1810.0, 10000.0),
                max_loaded_chunks: 2048,
                world_gen: shared::WorldGenSettings::PracticeRange,
                fixed_center: true,
            },
        },
    ];

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(lobbies)).into_response()
}