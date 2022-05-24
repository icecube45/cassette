//! Simple in-memory key/value store showing features of axum.

mod animation_pipeline;
mod mel_filter;


mod api;
//use api::create::create_animation;

#[macro_use]
extern crate enum_dispatch;

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
use hecs::{World, Entity, EntityBuilder};
use parking_lot::{Mutex, RwLock};
use core::time;
use std::{
    borrow::Cow,
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::{time::timeout, runtime::Runtime};
use std::thread;
mod dsp;

use crate::animation_pipeline::{pixel::Pixel, output::Output};

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
        // .route("/get_entity",
        //     get({
        //         let world = world.clone(); 
        //         move |body| { get_entity(world, body) }
        //     }))
        .route("/dsp_ws",
            get({
                let world = world.clone(); 
                move |ws, body| { dsp_ws_handler( ws, body, world) }
            }))
        .route("/mod_entity",
            put({
                let world = world.clone();
                move |body| { mod_entity(world, body) }
            }));
            
    let dsp_wrapper = dsp::DSPWrapper::new(world.clone());
    tokio::spawn(async move {
        let one_sec = time::Duration::from_millis(1000);

        loop {
            thread::sleep(one_sec);
            let world = world.read();
            println!("{:?}", world.len());
            //https://docs.rs/hecs/latest/hecs/struct.QueryBorrow.html#method.with
            // how to query for type in the system...
            for (id, pixel) in world.query::<&Pixel>()
                .with::<Output>()    
                .iter() {
                    println!("{:?}", id);
                    println!("{:?}", pixel);
                    }
        }
    });



    // Run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn spawn_entity(world: Arc<RwLock<World>>, extract::Json(payload): extract::Json<Pixel>) -> Json<Entity> {
    let mut world = world.write();
    let mut builder = EntityBuilder::new();
    builder.add(Pixel { r: payload.r, g: payload.g, b: payload.b });
    let entity = world.spawn(builder.build());

    println!("Spawned entity: {}", entity.id());
    Json(entity)
}

async fn get_entity(world: Arc<RwLock<World>>, extract::Json(entity): extract::Json<Entity>) -> Result<Json<Pixel>, StatusCode> {
    let world = world.read();
    let pixel = world.get::<Pixel>(entity);
    
    match pixel {
        Ok(pixel) => return Ok(Json(Pixel { r: pixel.r, g: pixel.g, b: pixel.b })),
        Err(err) => {
            println!("Error getting pixel: {:?}", err);
            return Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn mod_entity(world: Arc<RwLock<World>>, extract::Json(entity): extract::Json<Entity>) -> StatusCode {
    let mut world = world.write();
    println!("Modified entity: {}", entity.id());
    match world.insert_one(entity, Output { name: "test".to_string(), width: 100, height: 100 }) {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            println!("Error inserting pixel: {:?}", err);
            StatusCode::NOT_FOUND
        }
    }
    
}


async fn dsp_ws_handler(
    ws: WebSocketUpgrade, 
    user_agent: Option<TypedHeader<headers::UserAgent>>, 
    state: Arc<RwLock<World>>
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }
    ws.on_upgrade({
        |ws| dsp_handle_socket(ws, state)
    })
    
}

async fn dsp_handle_socket(mut socket: WebSocket, world: Arc<RwLock<World>>) {
    // while let Some(msg) = socket.recv().await {
    //     let msg = if let Ok(msg) = msg {
    //         msg
    //     } else {
    //         // client disconnected
    //         return;
    //     };
    // }
    let mut world = world.write();
    let socket = Arc::new(Mutex::new(socket));

    world.spawn(({socket.clone()},));
}
