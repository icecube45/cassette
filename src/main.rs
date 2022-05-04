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
                println!("{:?}", state.lock().unwrap().db);
            }
        });

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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
        println!("`{}` connected", user_agent.as_str());
    }
    ws.on_upgrade({
        |ws| handle_socket(ws, state)
    })
    
}

async fn handle_socket(mut socket: WebSocket, state: Arc<Mutex<State>>) {
    if let Ok(msg) = timeout(Duration::from_secs(5), socket.recv()).await {
        if let Some(msg) = msg {
            match msg {
                Ok(msg) => {
                    match msg {
                        Message::Text(t) => {
                            println!("client requested output: {:?}", t);
                        }
                        Message::Binary(_) => {
                            println!("client sent binary data");
                        }
                        Message::Ping(_) => {
                            println!("socket ping");
                        }
                        Message::Pong(_) => {
                            println!("socket pong");
                        }
                        Message::Close(_) => {
                            println!("client disconnected");
                            return;
                        }
                    }
                }
                Err(_) => {
                    println!("something happends");
                    return;
                }
            }
         }
        } else {
            //prob do some pattern matching on Err(_) to catch what kind of error happend
            println!("timeout");
            return;
           }

    loop {
        let state = Arc::clone(&state);
        let message = state.lock().unwrap().db.len().to_string();
        if socket.send(Message::Text(String::from(message))).await.is_err()
        {
            println!("client disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

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