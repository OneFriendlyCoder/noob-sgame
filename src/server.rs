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
    // Stores all active players in the game.
    // Each player struct would typically include:
    // position, speed, score, targets hit, ammo, etc.
    // This acts as the single source of truth for player data.

    enemies: Vec<Vec<player>>,      
    // Enemies per player. Each player has their own enemy list.
    // This allows each player to have personalized challenges.
    // Enables asynchronous gameplay and targeted enemy logic.
}

struct Client {     
    id: Uuid,               
    // Unique identifier for each connected client.
    // Helps in managing client connections and tracking their state.

    ws: WebSocket,          
    // Persistent websocket connection.
    // Allows continuous communication with the client
    // without reconnecting every frame.

    last_seen: Instant,     
    // Timestamp of last activity from the client.
    // Useful for detecting timeouts or disconnected clients.
}

struct ServerState{
    global_state: Arc<RwLock<GlobalState>>,
    // Shared, thread-safe global game state.
    // Arc<RwLock<>> allows multiple threads to read the game state
    // concurrently, while writes are exclusive.

    clients: HashMap<Uuid, Client>,
    // Stores all connected clients.
    // Keyed by UUID for easy lookup, adding, or removing clients.
}

// Handles incoming client connections.
// Should accept websocket connections, verify credentials (game code/password),
// and register clients in ServerState.
pub fn handleClientConnections() {
    // Steps to implement:
    // 1. Accept a TCP/WebSocket connection from a client.
    // 2. Authenticate the client using game code/password.
    // 3. Add the client to ServerState.clients.
    // 4. Spawn a dedicated thread or async task for processing this client's inputs.
}

// Main server loop.
// Responsible for initializing game state, waiting for all players to connect,
// and orchestrating game state updates and broadcasts.
pub fn run_Server() {
    // Steps to implement:
    // 1. Wait for all expected clients to connect.
    // 2. Initialize players and their enemies in GlobalState.
    // 3. Initialize the ServerState with GlobalState and clients.
    // 4. Start the main game loop:
    //      - Process inputs from clients.
    //      - Update GlobalState (player positions, enemy behavior, collisions, etc.).
    //      - Broadcast game state updates (or diffs) to all connected clients.
}

// Main entry point.
async fn main() {
    println!("Run as server or client? (s/c) : ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().to_lowercase();

    match choice.as_str() {
        "s" => run_Server.await(),  // Start server mode
        "c" => run_Client.await(),  // Start client mode
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
