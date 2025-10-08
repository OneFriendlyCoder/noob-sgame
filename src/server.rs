// Game server: 

// accept client connections (websockets)
// authorize the player with the game code and password
// spawn a thread for each player -> each thread will correspond to an enemy
// initialize the player's (position, speed, targets shot etc)
// starts changing the game state based on the inputs of the player
// render those changes in the game
// define a struct called game state
// the server should broadcast the game state each frame(optimization: only the diffs) to all the clients after the changes are made

// defining global state

use macroquad::prelude::*;
use crate::player::*;
use std::io::{self, Write};
use tokio::time::{self, Duration};
// use serde_json;
// use serde::Serialize;
use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::WebSocketStream;
use futures::stream::*;
use futures::SinkExt;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use tokio::time::Instant;


struct GlobalState {
    players: Vec<Player>, 
    enemies: Vec<Vec<Player>>,      

}

struct Client {     
    id: Uuid,               
    ws_w: SplitSink<WebSocketStream<TcpStream>, Message>,
    ws_r: SplitStream<WebSocketStream<TcpStream>>,        
    last_seen: Instant,     
}

struct ServerState{
    global_state: Arc<RwLock<GlobalState>>,
    clients: RwLock<HashMap<Uuid, Client>>,
}

async fn handleClientConnections(s:TcpStream, state: Arc<ServerState>) {
    let ws = match accept_async(s).await{
        Ok(ws) => ws,
        Err(e) => {
            println!("Websocket handshake failed : {}", e);
            return;
        }
    };

    let client_id = Uuid::new_v4();
    // println!("New client : {}", client_id);

    let(mut w, mut r) = ws.split();
    let c = Client{
        id: client_id,
        ws_r: r,
        ws_w: w,
        last_seen: Instant::now(),
    };
    let mut clients=  state.clients.write().unwrap();
    clients.insert(client_id, c);
    let mut gs = state.global_state.write().unwrap();
    let new_player = Player::new(
    client_id,
    vec3(0.0, 0.0, 0.0), 
    vec3(0.0, 0.0, 0.0), 
    "Player1".to_string(), 
    "Pistol".to_string(),  
    0.0,                 
    0.0                  
    );
    gs.players.push(new_player);
}


pub async fn run_Server() {
    let addr = "127.0.0.1:9001";
    let l = TcpListener::bind(addr).await.unwrap();
    println!("Server running at : {}", addr);

    let global_state = Arc::new(RwLock::new(GlobalState{
        players: Vec::new(),
        enemies: Vec::new(),
    }));

    let server_state = Arc::new((ServerState{
        global_state: Arc::clone(&global_state),            // Arc::clone increase the ref count of the Arc<T>
        clients: RwLock::new(HashMap::new()),
    }));

    let sc = Arc::clone(&server_state);
    tokio::spawn(async move{
        loop{
            if let Ok((stream, _)) = l.accept().await{
                let state_for_client = Arc::clone(&sc);
                tokio::spawn(async move{
                    handleClientConnections(stream, state_for_client).await;
                });
            }
        }
    });


    let mut ticker = time::interval(Duration::from_millis(1000/60));
    loop{
        ticker.tick().await;
        let snapshot = {
            let gs = global_state.read().unwrap();
            let mut buf = Vec::new();
            for p in &gs.players {
                buf.extend_from_slice(&p.position.x.to_le_bytes());
                buf.extend_from_slice(&p.position.y.to_le_bytes());
                buf.extend_from_slice(&p.position.z.to_le_bytes());
            }

            let mut clients = server_state.clients.write().unwrap(); // need write access for mutability
            for (id, client) in clients.iter_mut() {
                if let Err(e) = client.ws_w.send(Message::Binary(buf.clone().into())).await {
                    eprintln!("Failed to send to {} : {}", id, e);
                }
            }
        };
    }

}
