use shared::constants::HOST_IP;
use shared::network::{ClientMessage, ServerMessage};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc as tokio_mpsc;

pub async fn spawn_client() -> (
    tokio_mpsc::UnboundedSender<ClientMessage>,
    tokio_mpsc::Receiver<ServerMessage>,
) {
    let (game_tx, mut network_rx) = tokio_mpsc::unbounded_channel::<ClientMessage>();
    let (network_tx, game_rx) = tokio_mpsc::channel::<ServerMessage>(1024);
    
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let ip = HOST_IP;
    if let Err(e) = socket.connect(format!("{ip}:8080")).await {
        eprintln!("Failed to connect UDP socket: {}", e);
        return (game_tx, game_rx);
    }
    
    let socket = Arc::new(socket);
    let socket_rx = socket.clone();
    let socket_tx = socket.clone();
    let network_tx_clone = network_tx.clone();
    
    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            if let Ok(len) = socket_rx.recv(&mut buf).await {
                if let Ok(server_message) = bincode::deserialize::<ServerMessage>(&buf[..len]) {
                    if network_tx_clone.send(server_message).await.is_err() {
                        break;
                    }
                }
            }
        }
    });
    
    tokio::spawn(async move {
        while let Some(msg) = network_rx.recv().await {
            let encoded = bincode::serialize(&msg).unwrap();
            let _ = socket_tx.send(&encoded).await;
        }
    });
    
    (game_tx, game_rx)
}
