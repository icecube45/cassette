//! Simple in-memory key/value store showing features of axum.

use axum::{
    body::Bytes,
    error_handling::HandleErrorLayer,
    extract::{ContentLengthLimit, Path,
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router
    
};
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
use tokio::time::timeout;
use serde::{Serialize, Deserialize};

use std::thread;

use cassette_backend::RainbowWheel;
use cassette_backend::ExpandingSquares;
use cassette_backend::Mixer;
use cassette_backend::MixMode;
use cassette_backend::Animation;



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
    let state = Arc::new(Mutex::new(State::default()));

    // Build our application by composing routes
    let app = Router::new()
        .route(
            "/:key",
            // Add compression to `kv_get`
            post({
                let shared_state = Arc::clone(&state);
                move |path, body| kv_set(path, body, Arc::clone(&shared_state))
            }),
        )
        .route(
            "/ws",
            get({
                let shared_state = Arc::clone(&state);
                move |ws, user_agent| ws_handler(ws, user_agent, Arc::clone(&shared_state))
            }),
        )
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

    //spawn thread that will periodically print the shared state
    thread::spawn(move || 
        {
            let one_sec = Duration::from_secs(1);
            loop 
            {
                thread::sleep(one_sec);
                println!("State: {:?}", state.lock().unwrap().db);
            }
        });

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 80));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Default)]
struct State {
    db: HashMap<String, Bytes>,
}

async fn ws_handler(
    ws: WebSocketUpgrade, 
    user_agent: Option<TypedHeader<headers::UserAgent>>, 
    state: Arc<Mutex<State>>
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("Websocket Client connected: `{}`", user_agent.as_str());
    }
    ws.on_upgrade({
        |ws| handle_socket(ws, state)
    })
    
}

async fn handle_socket(mut socket: WebSocket, state: Arc<Mutex<State>>) {
    
    if let Ok(msg) = timeout(Duration::from_secs(5), socket.recv()).await { // timeout after 5 seconds
        if let Some(msg) = msg {                                   // if we received a message of Some type
            match msg {                                           // perform pattern matching on the message
                Ok(msg) => {                                      // if the message is of type OK
                    match msg { // match the message
                        Message::Text(t) => {
                            if t.len() > 1 {
                                println!("Client is too chatty :(");
                            }
                            println!("Client is live view of output {:?}", t);
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

    let mut rainbow = RainbowWheel::new();
    let mut squares = ExpandingSquares::new();
    let mut mixer = Mixer{
        mix_mode: MixMode::Progressive,
        mix_weight: 50f32,
    };
    loop {
        let rainbowFrame = rainbow.generate_frame(30, 10);
        let squaresFrame = squares.generate_frame(30, 10);
        let frame = mixer.mix(rainbowFrame, squaresFrame);




        let mut json_frame: String = "[".to_string();

        for j in 0..frame.height() {
            for i in 0..frame.width() {
                let pixel = frame.pixels[[j, i]];
                let r = pixel.r;
                let g = pixel.g;
                let b = pixel.b;
                json_frame.push_str(&format!("{{\"r\":{},\"g\":{},\"b\":{}, \"patched\":true}},", r, g, b));
            }
        }
        //remove last comma
        json_frame.pop();
        json_frame.push_str("]");
        if socket.send(Message::Text(json_frame)).await.is_err() {
            println!("Client disconnected :(");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    }
}






        

// fn rgb_to_pixel_entry(rgb: (u8, u8, u8), patched: bool) ):
//     print(patched)
//     return {
//         "r": rgb[0],
//         "g": rgb[1],
//         "b": rgb[2],
//         "patched": patched
//     }


async fn kv_set(Path(key): Path<String>, ContentLengthLimit(bytes): ContentLengthLimit<Bytes, { 1024 * 5_000 }>, state: Arc<Mutex<State>>) {
    let mut state = state.lock().unwrap();
    state.db.insert(key, bytes);
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