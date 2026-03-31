use shared::network::{ClientMessage, ServerMessage};
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use tokio::net::UdpSocket;
use tokio::sync::mpsc as tokio_mpsc;
pub fn spawn_client() -> (
    tokio_mpsc::UnboundedSender<ClientMessage>,
    mpsc::Receiver<ServerMessage>,
) {
    let (game_tx, mut network_rx) = tokio_mpsc::unbounded_channel::<ClientMessage>();
    let (network_tx, game_rx) = mpsc::channel::<ServerMessage>();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
            let ip = "127.0.0.1";
            if let Err(e) = socket.connect(format!("{ip}:8080")).await {
                eprintln!("Failed to connect UDP socket: {}", e);
                return;
            }
            let socket = Arc::new(socket);
            let socket_rx = socket.clone();
            let socket_tx = socket.clone();
            let network_tx_clone = network_tx.clone();
            let mut reader_task = tokio::spawn(async move {
                let mut buf = [0; 1024];
                loop {
                    if let Ok(len) = socket_rx.recv(&mut buf).await {
                        if let Ok(server_message) =
                            bincode::deserialize::<ServerMessage>(&buf[..len])
                        {
                            if network_tx_clone.send(server_message).is_err() {
                                break;
                            }
                        }
                    }
                }
            });
            let mut writer_task = tokio::spawn(async move {
                while let Some(msg) = network_rx.recv().await {
                    let encoded = bincode::serialize(&msg).unwrap();
                    let _ = socket_tx.send(&encoded).await;
                }
            });
            tokio::select! {
                _ = &mut reader_task => writer_task.abort(),
                _ = &mut writer_task => reader_task.abort(),
            }
            println!("Network thread shutting down.");
        });
    });
    (game_tx, game_rx)
}
