// server client implementation async

use tokio_tungstenite::*;
use futures::{StreamExt, SinkExt};
use tungstenite::protocol::Message;
use tokio::net::*;


// s is raw tcp connection, which is then upgraded to wss 
async fn handle_Connection(s: tokio::net::TcpStream){
    let wss = accept_async(s).await.unwrap();
    let (mut w, mut r) = wss.split();
    tokio::spawn(async move{
        while let Some(msg) = r.next().await{
            match msg {
                Ok(Message::Text(text)) => {        //pattern matching for Ok and not error
                    println!("Message Received : {}", text);
                }
                _ => {}
            }
        }
    });
}

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server is running at 127.0.0.1:8080");
    while let Ok((s, _addr)) = listener.accept().await{
        tokio::spawn(async move{
            handle_Connection(s).await;
        });
    }
}
