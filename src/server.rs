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

mod player;

use crate::player::*;
use std::io::{self, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, connect_async, tungstenite::protocol::Message};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use tokio::time::Instant;

#[derive(Debug)]
struct GlobalState {
    players: Vec<player>, 
    enemies: Vec<Vec<player>>,      

}

struct Client {     
    id: Uuid,               
    ws_r: ReadHalf<WebSocketStream<TcpStream>>,
    ws_w: WriteHalf<WebSocketStream<TcpStream>>,         
    last_seen: Instant,     
}

struct ServerState{
    global_state: Arc<RwLock<GlobalState>>,
    clients: HashMap<Uuid, Client>,
}

async fn handleClientConnections(s:TcpStream, state: Arc<ServerState>) {
    let ws = match accept_async(s).await{
        Ok(ws) => ws,
        Err(e) => {
            println!("Websocket handshake failed : {}"e);
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
    state.client.write().unwrap().insert(client_id, client);
    let mut gs = state.global_state.write().unwrap();
    gs.players.push(Player::new(client_id));
}

pub fn run_Server() {
    let addr = "127.0.0.1:9001";
    let l = TcpListener::bind(addr).await.unwrap();
    prinln!("Server running at : {}", addr);

    let global_state = Arc::new(RwLock::new(GlobalState{
        players: Vec::new(),
        enemies: Vec::new(),
    }));

    let server_state = Arc::new(RwLock::new(ServerState{
        global_state: Arc::clone(&global_state),            // Arc::clone increase the ref count of the Arc<T>
        clients: Arc::new(RwLock::new(HashMap::new())),
    }));

    let sc = Arc::clone(&server_state);
    tokio::spawn(async move{
        loop{
            if let Ok((stream, _)) = l.accept.await{
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
            serde_json::to_string(&*gs).unwrap_or_else(|_| "{}".to_string())
        };

        let clients = server_state.clients.read().unwrap();
        for (id, client) in clients.iter(){
            // Only Send diffs of the data
            if let Err(e) = client.ws_wr.send(Message::Text(snapshot.clone())).await{
                eprintln!("Failed to send to {} : {}", id, e);
            }
        }
    }

}

// Main entry point.
async fn main() {
    println!("Run as server or client? (s/c) : ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "s" => run_Server.await(),
        "c" => run_Client.await(),
        _ => println!("Invalid Choice, Please enter 's' or 'c'"),
    }

    loop {
        // Represents the game loop ticking frame by frame.
        // next_frame.await() would typically:
        // 1. Wait for the duration of one frame (e.g., 16ms for 60 FPS)
        // 2. Process queued client actions
        // 3. Update global game state
        // 4. Broadcast updates/diffs to all clients
        next_frame.await();
    }
}
