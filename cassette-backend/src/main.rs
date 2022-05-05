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

    loop {
        let pixels = rainbow(false);
        let mut frame: String = "[".to_string();
        let mut it = pixels.iter().peekable();
        while let Some(pixel) = it.next() {
            // serialize it and build json array
            let json = serde_json::to_string(&pixel).unwrap();
            // add it to frame
            frame.push_str(&json);
            // add comma if not last element
            if it.peek().is_some() {
                frame.push_str(",");
            }
        }
        frame.push_str("]");
        if socket.send(Message::Text(frame)).await.is_err() {
            println!("Client disconnected :(");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    }
}

fn wheel(mut n: u8) -> (u8, u8, u8) {
    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;
    if n < 85 {
        r = n * 3;
        g = 255-n*3;
        b = 0;
    }
    else if n < 170 {
        n = n-85;
        r = 255 - n*3;
        g = 0;
        b = n*3;
    }
    else {
        n = n-170;
        r = 0;
        g = n*3;
        b = 255 - n*3;
    }
    return (r, g, b);
}

static mut STEP: u16 = 0;
const WIDTH: u16 = 30;
const HEIGHT: u16 = 10;
const NUM_PIXELS: u16 = (WIDTH*HEIGHT);

#[derive(Copy, Clone, Serialize, Deserialize)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    patched: bool,
}


fn strip_to_matrix(strip: &mut [Pixel; NUM_PIXELS as usize]) -> &mut [Pixel; NUM_PIXELS as usize] {
    let mut j = 0;
    for i in 0..HEIGHT-1 {
        j = i + 1;
        strip.copy_within(0..WIDTH as usize, (j*WIDTH) as usize)
    }
    return strip;
}

fn rainbow(matrix: bool) -> [Pixel; NUM_PIXELS as usize] {
    let mut num_pixels_override = NUM_PIXELS;
    let mut pixels: [Pixel; NUM_PIXELS as usize] = [Pixel { r: 0, g: 0, b: 0, patched: false }; NUM_PIXELS as usize];
    if !matrix {
        num_pixels_override = WIDTH;
    }
    for i in 0..NUM_PIXELS as u32 {
        unsafe{
            let pixel_index: u32  = (i*256/num_pixels_override as u32) + STEP as u32;
            let (r, g, b) = wheel(pixel_index as u8);
            pixels[i as usize] = Pixel { r, g, b, patched: true };
        }
    }
    unsafe{
        STEP = STEP + 1;
        if STEP == 256 {
            STEP = 0;
        }
    }
    if !matrix{
        pixels = *strip_to_matrix(&mut pixels);
    }
    return pixels;
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