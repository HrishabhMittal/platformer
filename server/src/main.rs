use shared::constants::TICK_PER_SECOND;
use shared::network::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, mpsc};
use tokio::time::{self, Duration};

struct PlayerState {
    x: f32,
    y: f32,
    cursor_x: f32,
    cursor_y: f32,
    last_seen: tokio::time::Instant,
}

struct GlobalState {
    players: HashMap<u32, PlayerState>,
}

type ClientDirectory = Arc<Mutex<HashMap<u32, SocketAddr>>>;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0";
    let port = 8080;
    let address = format!("{host}:{port}");
    println!("UDP Server listening on {address}");

    let socket = Arc::new(UdpSocket::bind(address).await.unwrap());

    let (tx_input, rx_input) = mpsc::channel::<ClientObject>(1024);
    let (tx_bcast, mut rx_bcast) = mpsc::channel::<ServerMessage>(1024);

    let client_directory: ClientDirectory = Arc::new(Mutex::new(HashMap::new()));

    tokio::spawn(async move {
        game_tick_loop(rx_input, tx_bcast).await;
    });

    let socket_sender = socket.clone();
    let dir_sender = client_directory.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx_bcast.recv().await {
            let encoded = bincode::serialize(&msg).unwrap();
            let mut directory = dir_sender.lock().await;

            for (_, addr) in directory.iter() {
                let _ = socket_sender.send_to(&encoded, *addr).await;
            }

            if let ServerMessage::PlayerLeft { id } = msg {
                directory.remove(&id);
            }
        }
    });

    let mut buf = [0; 1024];
    let mut next_id: u32 = 0;
    let mut addr_to_id: HashMap<SocketAddr, u32> = HashMap::new();

    loop {
        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
            let id = if let Some(&existing_id) = addr_to_id.get(&addr) {
                existing_id
            } else {
                println!("New connection from {}", addr);
                let new_id = next_id;
                next_id += 1;

                addr_to_id.insert(addr, new_id);
                client_directory.lock().await.insert(new_id, addr);

                let welcome_msg = ServerMessage::NotifyId { id: new_id };
                let encoded = bincode::serialize(&welcome_msg).unwrap();
                let _ = socket.send_to(&encoded, addr).await;

                new_id
            };

            if let Ok(client_message) = bincode::deserialize::<ClientMessage>(&buf[..len]) {
                let obj = ClientObject { id, client_message };
                let _ = tx_input.send(obj).await;
            }
        }
    }
}

async fn game_tick_loop(
    mut rx_input: mpsc::Receiver<ClientObject>,
    tx_bcast: mpsc::Sender<ServerMessage>,
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
        let mut to_remove = Vec::new();
        for (&id, player) in &state.players {
            if player.last_seen.elapsed().as_secs() > 5 {
                let _ = tx_bcast.send(ServerMessage::PlayerLeft { id }).await;
                to_remove.push(id);
            } else {
                let _ = tx_bcast
                    .send(ServerMessage::PlayerMoved {
                        id,
                        position: Vec2::new(player.x, player.y),
                    })
                    .await;
                let _ = tx_bcast
                    .send(ServerMessage::PlayerMouseMoved {
                        id,
                        position: Vec2::new(player.cursor_x, player.cursor_y),
                    })
                    .await;
            }
        }
        for i in to_remove {
            let _ = &state.players.remove(&i);
        }
    }
}

fn apply_client_obj(state: &mut GlobalState, obj: ClientObject) {
    let player = state.players.entry(obj.id).or_insert(PlayerState {
        x: 0.0,
        y: 0.0,
        cursor_x: 0.0,
        cursor_y: 0.0,
        last_seen: tokio::time::Instant::now(),
    });
    match obj.client_message {
        ClientMessage::PositionUpdate { position } => {
            player.x = position.x;
            player.y = position.y;
            player.last_seen = tokio::time::Instant::now();
        }
        ClientMessage::MouseUpdate { position } => {
            player.cursor_x = position.x;
            player.cursor_y = position.y;
            player.last_seen = tokio::time::Instant::now();
        }
    }
}
