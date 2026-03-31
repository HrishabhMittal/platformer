use std::collections::HashMap;
use tokio::time;
use tokio::time::Duration;

use shared::constants::TICK_PER_SECOND;
use shared::network::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc};

struct PlayerState {
    x: f32,
    y: f32,
    cursor_x: f32,
    cursor_y: f32,
}

struct GlobalState {
    players: HashMap<u32, PlayerState>,
}

#[tokio::main]
async fn main() {
    let host = "127.0.0.1";
    let port = 8080;
    let address = format!("{host}:{port}");
    println!("listening on {address}");
    let listener = TcpListener::bind(address).await.unwrap();
    let (tx, rx) = mpsc::channel::<ClientObject>(1024);
    let (broadcast_tx, _) = broadcast::channel::<ServerMessage>(1024);
    let mut id: u32 = 0;
    let tx_bcast_clone = broadcast_tx.clone();
    tokio::spawn(async move {
        game_tick_loop(rx, tx_bcast_clone).await;
    });
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let tx_copy = tx.clone();
        let broadcast_tx = broadcast_tx.subscribe();
        tokio::spawn(async move {
            client_handler(id, socket, tx_copy, broadcast_tx).await;
        });
        id += 1;
    }
}
async fn game_tick_loop(
    mut rx_input: mpsc::Receiver<ClientObject>,
    tx_bcast: broadcast::Sender<ServerMessage>,
) {
    let tick_duration = Duration::from_secs_f32(1.0 / TICK_PER_SECOND);
    let mut interval = time::interval(tick_duration);

    let mut state = GlobalState {
        players: HashMap::new(),
    };

    loop {
        interval.tick().await;
        while let Ok(msg) = rx_input.try_recv() {
            apply_client_obj(&mut state, msg);
        }
        for (&id, player) in &state.players {
            let _ = tx_bcast.send(ServerMessage::PlayerMoved {
                id,
                position: bevy_math::Vec2::new(player.x, player.y),
            });
        }
    }
}
async fn client_handler(
    id: u32,
    socket: TcpStream,
    sender: mpsc::Sender<ClientObject>,
    mut bcast: broadcast::Receiver<ServerMessage>,
) {
    println!("new connection: {id}");
    let (mut reader, mut writer) = socket.into_split();

    let mut writer_end = tokio::spawn(async move {
        let id = ServerMessage::NotifyId { id };
        let msg = bincode::serialize(&id).unwrap();
        if writer.write_all(&msg).await.is_err() {
            return;
        }
        while let Ok(msg) = bcast.recv().await {
            let encoded = bincode::serialize(&msg).unwrap();
            if writer.write_all(&encoded).await.is_err() {
                break;
            }
        }
    });

    let mut reader_end = tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            match reader.read(&mut buf[..]).await {
                Ok(0) => break,
                Ok(n) => {
                    if let Ok(client_message) = bincode::deserialize::<ClientMessage>(&buf[..n]) {
                        let msg = ClientObject { id, client_message };
                        if sender.send(msg).await.is_err() {
                            break;
                        }
                    }
                }
                Err(_) => break,
            }
        }
    });
    tokio::select! {
        _ = &mut reader_end => writer_end.abort(),
        _ = &mut writer_end => reader_end.abort(),
    }
    println!("client disconnected: {id}");
}
fn apply_client_obj(state: &mut GlobalState, obj: ClientObject) {
    let player = state.players.entry(obj.id).or_insert(PlayerState {
        x: 0.0,
        y: 0.0,
        cursor_x: 0.0,
        cursor_y: 0.0,
    });
    match obj.client_message {
        ClientMessage::PositionUpdate { position } => {
            player.x = position.x;
            player.y = position.y;
        }
        ClientMessage::MouseUpdate { position } => {
            player.cursor_x = position.x;
            player.cursor_y = position.y;
        }
    }
}
