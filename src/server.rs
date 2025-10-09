use crate::player::*;
use macroquad::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use uuid::Uuid;
use tokio::time::{self, Duration, Instant};

#[derive(Clone)]
pub struct GlobalState {
    pub players: Vec<Player>,
    pub enemies: Vec<Vec<Player>>,
}

pub struct Client {
    pub id: Uuid,
    pub ws_w: Arc<Mutex<futures::stream::SplitSink<WebSocketStream<TcpStream>, Message>>>,
    pub ws_r: futures::stream::SplitStream<WebSocketStream<TcpStream>>,
    pub last_seen: Instant,
}

pub struct ServerState {
    pub global_state: Arc<RwLock<GlobalState>>,
    pub clients: RwLock<HashMap<Uuid, Client>>,
}

async fn handle_client_connections(stream: TcpStream, state: Arc<ServerState>) {
    let ws = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    let client_id = Uuid::new_v4();
    let (w, r) = ws.split();
    let client = Client {
        id: client_id,
        ws_r: r,
        ws_w: Arc::new(Mutex::new(w)),
        last_seen: Instant::now(),
    };

    state.clients.write().unwrap().insert(client_id, client);

    let mut gs = state.global_state.write().unwrap();
    let new_player = Player::new(
        client_id,
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 0.0, 0.0),
        format!("Player-{}", gs.players.len()),
        "Shotgun".to_string(),
        0.0,
        0.0,
    );
    gs.players.push(new_player);

    // rebuild enemies
    gs.enemies = (0..gs.players.len())
        .map(|i| {
            gs.players
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, p)| p.clone())
                .collect::<Vec<Player>>()
        })
        .collect();
}

pub async fn run_server(server_state: Arc<ServerState>) {
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running at: {}", addr);

    let server_clone = Arc::clone(&server_state);
    tokio::spawn(async move {
        loop {
            if let Ok((stream, _)) = listener.accept().await {
                let state_for_client = Arc::clone(&server_clone);
                tokio::spawn(async move {
                    handle_client_connections(stream, state_for_client).await;
                });
            }
        }
    });

    let mut ticker = time::interval(Duration::from_millis(1000 / 60));
    loop {
        ticker.tick().await;

        // clone player positions first to avoid holding the lock across .await
        let player_positions: Vec<Vec3> = {
            let gs = server_state.global_state.read().unwrap();
            gs.players.iter().map(|p| p.position).collect()
        };

        let mut buf = Vec::new();
        for pos in player_positions {
            buf.extend_from_slice(&pos.x.to_le_bytes());
            buf.extend_from_slice(&pos.y.to_le_bytes());
            buf.extend_from_slice(&pos.z.to_le_bytes());
        }

        // clone client Arc<Mutex<...>> handles
        let clients: Vec<Arc<Mutex<futures::stream::SplitSink<WebSocketStream<TcpStream>, Message>>>> =
            {
                let clients_map = server_state.clients.read().unwrap();
                clients_map.values().map(|c| Arc::clone(&c.ws_w)).collect()
            };

        // send to clients without holding HashMap lock
        for client_ws in clients {
            let mut sink = client_ws.lock().await;
            if let Err(e) = sink.send(Message::Binary(buf.clone().into())).await {
                eprintln!("Failed to send to client: {}", e);
            }
        }
    }
}
