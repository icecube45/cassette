//! Simple in-memory key/value store showing features of axum.

use axum::{
    body::Bytes,
    error_handling::HandleErrorLayer,
    extract::{ContentLengthLimit, Path,
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader, self
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
    Json
    
};
use hecs::{World, Entity, Bundle, EntityBuilder};
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
        .route("/get_entity",
            get({
                let world = world.clone(); 
                move |body| { get_entity(world, body) }
            }));

    tokio::spawn(async move {
        let one_sec = time::Duration::from_millis(1000);
        loop {
            thread::sleep(one_sec);
            let world = world.read().await;
            println!("{:?}", world.len());
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

#[derive(Bundle, Deserialize, Debug, Serialize)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

async fn spawn_entity(world: Arc<RwLock<World>>, extract::Json(payload): extract::Json<Pixel>) -> Json<Entity> {
    let mut world = world.write().await;
    let mut builder = EntityBuilder::new();
    builder.add(Pixel { r: payload.r, g: payload.g, b: payload.b });
    let entity = world.spawn(builder.build());
    println!("Spawned entity: {}", entity.id());
    Json(entity)
}

async fn get_entity(world: Arc<RwLock<World>>, extract::Json(entity): extract::Json<Entity>) -> Json<Pixel> {
    let world = world.read().await;
    let pixel = world.get::<Pixel>(entity);

    match pixel {
        Ok(pixel) => return Json(Pixel { r: pixel.r, g: pixel.g, b: pixel.b }),
        Err(err) => {
            println!("Error getting pixel: {:?}", err);
            return Json(Pixel { r: 0, g: 0, b: 0 });
        }
    }
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