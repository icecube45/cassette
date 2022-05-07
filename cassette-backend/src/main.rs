//! Simple in-memory key/value store showing features of axum.

pub mod components;

use axum::{
    body::Bytes,
    error_handling::HandleErrorLayer,
    extract::{ContentLengthLimit, Path,
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader, self
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
    Json
    
};
use core::time;
use std::{
    borrow::Cow,
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::{trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::{time::timeout, sync::RwLock};
use serde::{Serialize, Deserialize};
use bevy_ecs::prelude::*;
use bevy_ecs::world::World;


use std::thread;


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_key_value_store=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // initialize our shared state
    let world = Arc::new(RwLock::new(World::default()));

    // initialize the networking configuration
    let app = Router::new()
        .route("/spawn_entity",
            post({
                let world = world.clone();
                move |body| { spawn_entity(world, body) }
            }))
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    tokio::spawn(async move {
        let one_sec = time::Duration::from_millis(16);
        loop {
            thread::sleep(one_sec);
            let world = world.read().await;
            println!("{:?}", world.entities().len());
        }
    });


    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Component)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Serialize)]
struct EntityID {
    id: u32
}

async fn spawn_entity(world: Arc<RwLock<World>>, extract::Json(payload): extract::Json<Pixel>) -> Json<EntityID> {
    let mut world = world.write().await;
    let entity = world.spawn()
        .insert(Pixel {
            r: payload.r,
            g: payload.g,
            b: payload.b
        }).id();
    println!("Spawned entity: {}", entity.id());
    Json(EntityID { id: entity.id() })
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}