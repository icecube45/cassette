//! Simple in-memory key/value store showing features of axum.

mod animation_pipeline;
mod mel_filter;


mod api;
//use api::create::create_animation;

#[macro_use]
extern crate enum_dispatch;

use animation_pipeline::{output, mixer};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader, self, Path
    },
    http::{StatusCode, Method},
    response::IntoResponse,
    routing::{get, put},
    Router,
    Json
    
};
use tower_http::cors::{CorsLayer, any, Any};
use dsp::DSP;
use hecs::{World, Entity, EntityBuilder};
use parking_lot::{Mutex, RwLock};
use core::time;
use std::{
    net::SocketAddr,
    sync::Arc,
    time::Duration, collections::HashMap,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tokio::{time::timeout};
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

    let dsp_wrapper = dsp::DSPWrapper::new(world.clone());



    let cors_layer = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let api_routing = Router::new()
        .route("/outputs", get({
            let world = world.clone(); 
            move || { api_get_outputs(world) }
        }))
        .route("/output/:output_id/mixers/:mixer_id",
            get({
                let world = world.clone(); 
                move |ws, body| { dsp_ws_handler( ws, body, world) }
            }));
                


    // initialize the networking configuration
    let app = Router::new()
        // .route("/get_entity",
        //     get({
        //         let world = world.clone(); 
        //         move |body| { get_entity(world, body) }
        //     }))
        .nest("/api", api_routing)
        .route("/dsp_ws",
            get({
                let world = world.clone(); 
                move |ws, body| { dsp_ws_handler( ws, body, world) }
            }))
        .route(
            "/ws",
            get({
                let world = world.clone();
                let dsp_clone = dsp_wrapper.dsp.clone();
                move |ws, user_agent| ws_handler(ws, user_agent, world, dsp_clone)
            }),
        )
        .route("/mod_entity",
            put({
                let world = world.clone();
                move |body| { mod_entity(world, body) }
            }))
        .layer(cors_layer);
            
    // tokio::spawn(async move {
    //     let one_sec = time::Duration::from_millis(1000);

    //     // loop {
    //     //     thread::sleep(one_sec);
    //     //     let world = world.read();
    //     //     // println!("{:?}", world.len());
    //     //     //https://docs.rs/hecs/latest/hecs/struct.QueryBorrow.html#method.with
    //     //     // how to query for type in the system...
    //     //     for (id, pixel) in world.query::<&Pixel>()
    //     //         .with::<Output>()    
    //     //         .iter() {
    //     //             // println!("{:?}", id);
    //     //             // println!("{:?}", pixel);
    //     //             }
    //     // }
    // });

    println!("Creating one output");
    let dsp_clone = dsp_wrapper.dsp.clone();
    let output_entity = spawn_output(world.clone(), dsp_clone, 3);
    println!("Created output: {}", output_entity.id());
    println!("Creating one output");
    let dsp_clone = dsp_wrapper.dsp.clone();
    let output_entity = spawn_output(world.clone(), dsp_clone, 4);
    println!("Created output: {}", output_entity.id());
    let clone_world = world.clone();
    tokio::task::spawn_blocking(move || processing_thread(clone_world));



    // Run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


fn processing_thread(world: Arc<RwLock<World>>){
    loop {
        {
            let world = world.read();
            let mut query = world.query::<&Arc<Mutex<Output>>>();
            query.iter().for_each(|(entity, output)| {
                {
                    let mut output = output.lock();
                    if output.enabled() {
                        output.process();
                    }
                }
            });
        }
        thread::sleep(std::time::Duration::from_millis(10))
    }
}

async fn api_get_outputs(world: Arc<RwLock<World>>) -> Json<Vec<output::ApiRepresentation>>{
    let world = world.read();
    let mut query = world.query::<&Arc<Mutex<Output>>>();
    let mut response: Vec<output::ApiRepresentation> = Vec::new();
    query.iter().for_each(|(entity, output)| {
        let mut entity_resp = 
        {
            let mut output = output.lock();
            output.get_api_representation()
        };
        entity_resp.id = entity.to_bits().get();
        response.push(entity_resp);
    });
    Json(response)
}

async fn api_get_mixer(world: Arc<RwLock<World>>, output_id: u64, mixer_id: usize) -> Json<mixer::ApiRepresentation> {
    let world = world.read();
    let mut query = world.query::<&Arc<Mutex<Output>>>();
    let entity = Entity::from_bits(output_id).unwrap();
    {
        let output = world.get::<Arc<Mutex<Output>>>(entity).unwrap();

        let mut entity_resp = 
            {
                let mut output = output.lock();
                let api_rep = {
                    match mixer_id {
                            1 => output.mixer1.get_api_representation(),
                            2 => output.mixer2.get_api_representation(),
                            3 => output.masterMixer.get_api_representation(),
                            _ => output.masterMixer.get_api_representation(),
                    }
                };
                return Json(api_rep);
                // return Json(output.get_mixer_by_id(mixer_id).get_api_representation());
            };
            
    };
}


async fn mixer_get(Path(params): Path<HashMap<String, String>>) {
    let output_id = params.get("output_id");
    let mixer_id = params.get("mixer_id");
}


async fn spawn_entity(world: Arc<RwLock<World>>, extract::Json(payload): extract::Json<Pixel>) -> Json<Entity> {
    let mut world = world.write();
    let mut builder = EntityBuilder::new();
    builder.add(Pixel { r: payload.r, g: payload.g, b: payload.b });
    let entity = world.spawn(builder.build());

    println!("Spawned entity: {}", entity.id());
    Json(entity)
}


fn spawn_output(world: Arc<RwLock<World>>, dsp: Arc<Mutex<DSP>>, index: usize) -> Entity {
    let mut builder = EntityBuilder::new();
    let new_dsp = dsp.clone();
    builder.add(Arc::new(Mutex::new(Output::new(100, 100, new_dsp, index))));
    let mut world = world.write();
    let entity = world.spawn(builder.build());

    println!("Spawned output: {}", entity.id());
    entity
}

// async fn get_output(world: Arc<RwLock<World>>, extract::Json(entity): extract::Json<Entity>) -> Result<hecs::RefMut<'_, output::Output, >, StatusCode> {
//     let world = world.read();
//     let output = world.get_mut::<Output>(entity);
//     match output {
//         Ok(pixel) => return Ok(output.unwrap()),
//         Err(err) => {
//             println!("Error getting output: {:?}", err);
//             return Err(StatusCode::NOT_FOUND)
//         }
//     }
// }


async fn get_entity(world: Arc<RwLock<World>>, extract::Json(entity): extract::Json<Entity>) -> Result<Json<Pixel>, StatusCode> {
    let world = world.read();
    let pixel = world.get::<Pixel>(entity);
    
    match pixel {
        Ok(pixel) => return Ok(Json(Pixel { r: pixel.r, g: pixel.g, b: pixel.b })),
        Err(err) => {
            // println!("Error getting pixel: {:?}", err);
            return Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn mod_entity(world: Arc<RwLock<World>>, extract::Json(entity): extract::Json<Entity>) -> StatusCode {
    // let mut world = world.write();
    // println!("Modified entity: {}", entity.id());
    // match world.insert_one(entity, Output {
    //                             name: "test".to_string(), 
    //                             width: 100, 
    //                             height: 100 }) {
    //     Ok(_) => StatusCode::OK,
    //     Err(err) => {
    //         println!("Error inserting pixel: {:?}", err);
    //         StatusCode::NOT_FOUND
    //     }
    // }
    StatusCode::OK
    
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

async fn ws_handler(
    ws: WebSocketUpgrade, 
    user_agent: Option<TypedHeader<headers::UserAgent>>, 
    world: Arc<RwLock<World>>,
    dsp: Arc<Mutex<DSP>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("Websocket Client connected: `{}`", user_agent.as_str());
    }
    ws.on_upgrade({
        let world = world.clone();
        |ws| handle_socket(ws, world, dsp)
    })
    
}



async fn handle_socket(mut socket: WebSocket, world: Arc<RwLock<World>>, dsp: Arc<Mutex<DSP>>) {
    let mut entity_id = 0; 
    if let Ok(msg) = timeout(Duration::from_secs(5), socket.recv()).await { // timeout after 5 seconds
        if let Some(msg) = msg {                                   // if we received a message of Some type
            match msg {                                           // perform pattern matching on the message
                Ok(msg) => {                                      // if the message is of type OK
                    match msg { // match the message
                        Message::Text(t) => {
                            if t.len() > 10 {
                                println!("Client is too chatty :(");
                                return;
                            }
                            entity_id = t.parse::<u64>().unwrap();
                        }
                        Message::Binary(_) => {
                            println!("Client sent binary data - we weren't expecting this");
                        }
                        Message::Ping(_) => {
                            println!("socket ping");
                        }
                        Message::Pong(_) => {
                            println!("socket pong");
                        }
                        Message::Close(_) => {
                            println!("Client disconnected :(");
                            return;
                        }
                    }
                }
                Err(_) => { // if the message is of type Err
                    println!("We probably shouldn't see this"); // print something
                    return;
                }
            }
         }
        } else { // if we did not receive a message in the given timeframe then error
            //prob do some pattern matching on Err(_) to catch what kind of error happend
            println!("Client didn't welcome us :(");
            return;
           }


    println!("Got new frontend live view socket requesting view of output {}", entity_id);
    let socket = Arc::new(Mutex::new(socket));
    {
        let world_read = world.read();
        let entity = Entity::from_bits(entity_id).unwrap();
        {
            let entity_return = world_read.get::<Arc<Mutex<Output>>>(entity);
            match entity_return {
                Ok(output) => {
                    let mut output = output.lock();
                    output.add_websocket(socket);
                    output.enable();
                },
                Err(err) => {
                    println!("Error getting output: {:?}", err);
                    return;
                }
            }

        }
    }






    // loop {
    //     let frame3 = {
    //         let world_read = world.read();
    //         let x = world_read.get_mut::<Output>(output_entity).unwrap().process();
    //         x
    //     };
    //     // let frame3 = output.process();
    //     // println!("AH");
    //     let mut json_frame: String = "[".to_string();
    //     for pixel in frame3.pixels.iter() {
    //         json_frame.push_str(&format!("{},", pixel));
    //     }
    //     json_frame.pop();
    //     json_frame.push(']');
    //     socket.send(Message::Text(json_frame)).await.unwrap();
    //     // tokio::time::sleep(Duration::from_millis(15)).await;
    //     // let mut it = pixels.iter().peekable();
    //     // while let Some(pixel) = it.next() {
    //     //     // serialize it and build json array
    //     //     let json = serde_json::to_string(&pixel).unwrap();
    //     //     // add it to frame
    //     //     frame.push_str(&json);
    //     //     // add comma if not last element
    //     //     if it.peek().is_some() {
    //     //         frame.push_str(",");
    //     //     }
    //     // }
    //     // frame.push_str("]");
    //     // if socket.send(Message::Text(frame)).await.is_err() {
    //     //     println!("Client disconnected :(");
    //     //     return;
    //     // }
    //     // tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    // }
}


async fn dsp_handle_socket(socket: WebSocket, world: Arc<RwLock<World>>) {
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
